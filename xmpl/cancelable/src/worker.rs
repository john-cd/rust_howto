use std::thread;

use crossbeam::channel;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crossbeam::channel::TryRecvError;
use crossbeam::channel::select;

use super::task::TaskResponse;
use crate::*;

// Messages:
struct Die;

/// Maximum number of tasks that can be queued.
/// If this limit is reached, no new task will be queued.
const MAX_QUEUED_TASKS: usize = 5;

/// Manage a thread to execute tasks.
pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    s_die: Option<Sender<Die>>,       // Send the kill signal,
    cancel_source: CancelTokenSource, // Send interrupt to a task,
    s_task: Sender<Task>,             // Send tasks to the worker thread.
    r_taskresponse: Receiver<TaskResponse>,
}

impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker {
    pub fn new() -> Self {
        let (s_task, r_task) = channel::bounded::<Task>(MAX_QUEUED_TASKS);
        let (s_die, r_die) = channel::bounded::<Die>(1);
        let cancel_source = CancelTokenSource::new();
        let (s_taskresponse, r_taskresponse) = channel::unbounded::<TaskResponse>();

        let thread = {
            let cancel_source = cancel_source.clone();
            let s_taskresponse = s_taskresponse.clone();
            thread::spawn(move || {
                loop {
                    select! {
                                            recv(r_die) -> _ => {
                                                tracing::info!("Worker thread is stopping.");
                                                break;
                                            }
                                            recv(r_task) -> msg => {
                                                match msg {
                                                    Ok(task) => {
                                                        if !r_die.is_empty() { continue; }
                                                        let task_id = task.task_id();
                                                        let token = cancel_source.make_token(task_id);
                                                        tracing::info!("Task {task_id} starting.");
                                                        match task.execute(token, &s_taskresponse) {
                                                            Ok(final_task_response) => {
                                                                tracing::info!("Task {} completed.", final_task_response.task_id);
                                                            }
                                                            Err(e) => {
                                                                match e.downcast_ref::<TaskError>() {
                        Some(&TaskError::Interrupted) => { tracing::info!("Task {task_id} interrupted."); }
                        _ => { tracing::error!("Task error: {e:?}"); }
                    }

                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        tracing::error!("Channel error: {e}");
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                }
            })
        };
        Self {
            thread: Some(thread),
            s_die: Some(s_die),
            cancel_source,
            s_task,
            r_taskresponse,
        }
    }

    /// Request a task execution, unless too many of them are already
    /// queued.
    pub fn start(&self, task: Task) {
        let task_id = task.task_id();
        if self.s_task.try_send(task).is_ok() {
            tracing::info!("Queued task {}.", task_id);
        } else {
            tracing::warn!("Too many tasks in the queue, dropping task {}", task_id);
        }
    }

    /// Make the worker thread stop
    /// (interrupting the current task if any).
    pub fn die(&mut self) -> anyhow::Result<()> {
        self.cancel_source.cancel_all()?; // Interrupt current tasks if any.
        if let Some(sender) = self.s_die.take() {
            if let Err(e) = sender.send(Die) {
                tracing::error!("Die signal: channel error: {e}");
                return Err(e.into());
            }
        }
        if let Some(thread) = self.thread.take() {
            if thread.join().is_err() {
                tracing::error!("Child thread join() failed."); // Should not happen.
            } else {
                tracing::info!("Worker gracefully stopped.");
            }
        }
        Ok(())
    }

    pub fn process_responses(&self) {
        loop {
            match self.r_taskresponse.try_recv() {
                Ok(taskreponse) => {
                    println!("Processed a response from task {}.", taskreponse.task_id);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    tracing::error!("Error: {e}");
                    break;
                }
            }
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.die();
    }
}
