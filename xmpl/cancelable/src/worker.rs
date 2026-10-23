//! A worker thread that runs queued tasks.
//!
//! TODO Reviewed ??-??-2026.

use std::thread;

use crossbeam::channel;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crossbeam::channel::TrySendError;
use crossbeam::channel::select;

use super::cancel::CancelToken;
use super::task::TaskResponse;
use crate::*;

// Messages:
/// A request that makes the worker thread stop.
struct Die;

/// Maximum number of tasks that can be queued.
/// If this limit is reached, no new task will be queued.
const MAX_QUEUED_TASKS: usize = 5;

/// A task together with its registered cancellation token.
struct QueuedTask {
    /// The work to execute.
    task: Task,
    /// The cancellation flag registered before queueing.
    cancel_token: CancelToken,
}

/// Manage a thread to execute tasks.
pub struct Worker {
    /// The worker thread.
    thread: Option<thread::JoinHandle<()>>,
    /// Sends the kill signal.
    s_die: Option<Sender<Die>>, // Send the kill signal,
    /// Sends interrupt requests to the running task.
    cancel_source: CancelTokenSource, // Send interrupt to a task,
    /// Sends the tasks to the worker thread.
    s_task: Sender<QueuedTask>,
    /// Receives the progress reports of the tasks.
    r_taskresponse: Receiver<TaskResponse>,
}

impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker {
    /// Starts the worker thread.
    pub fn new() -> Self {
        let (s_task, r_task) = channel::bounded::<QueuedTask>(MAX_QUEUED_TASKS);
        let (s_die, r_die) = channel::bounded::<Die>(1);
        let cancel_source = CancelTokenSource::new();
        #[expect(
            clippy::disallowed_methods,
            reason = "responses are produced one per submitted task, which is itself bounded"
        )]
        let (s_taskresponse, r_taskresponse) = channel::unbounded::<TaskResponse>();

        let thread = {
            thread::spawn(move || {
                loop {
                    select! {
                        recv(r_die) -> _ => {
                            tracing::info!("Worker thread is stopping.");
                            break;
                        }
                        recv(r_task) -> msg => {
                            match msg {
                                Ok(queued) => {
                                    if !r_die.is_empty() {
                                        break;
                                    }
                                    let task_id = queued.task.task_id();
                                    tracing::info!("Task {task_id} starting.");
                                    match queued.task.execute(queued.cancel_token, &s_taskresponse) {
                                        Ok(final_task_response) => {
                                            tracing::info!("Task {} completed.", final_task_response.task_id);
                                        }
                                        Err(error) => {
                                        if error.downcast_ref::<TaskError>() == Some(&TaskError::Interrupted) {
                                            tracing::info!("Task {task_id} interrupted.");
                                        } else {
                                            tracing::error!("Task error: {error:?}");
                                            }
                                        }
                                    }
                                }
                                Err(error) => {
                                    tracing::error!("Task channel error: {error}");
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
    pub fn start(&self, task: Task) -> anyhow::Result<()> {
        let task_id = task.task_id();
        let cancel_token = self.cancel_source.make_token(task_id)?;
        let queued = QueuedTask { task, cancel_token };
        match self.s_task.try_send(queued) {
            Ok(()) => {
                tracing::info!("Queued task {task_id}.");
                Ok(())
            }
            Err(TrySendError::Full(_)) => {
                anyhow::bail!("task queue is full; task {task_id} was not queued")
            }
            Err(TrySendError::Disconnected(_)) => {
                anyhow::bail!("worker is unavailable; task {task_id} was not queued")
            }
        }
    }

    /// Requests cancellation of a queued or currently running task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task is not queued or running, or if the cancellation registry is poisoned.
    pub fn cancel_task(&self, task_id: u64) -> anyhow::Result<()> {
        self.cancel_source.cancel_task(task_id)
    }

    /// Make the worker thread stop
    /// (interrupting the current task if any).
    pub fn die(&mut self) -> anyhow::Result<()> {
        let cancel_result = self.cancel_source.cancel_all();
        if let Some(sender) = self.s_die.take()
            && sender.send(Die).is_err()
        {
            tracing::warn!("Worker had already stopped before receiving the stop signal.");
        }
        let join_result = if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|panic| anyhow::anyhow!("worker thread panicked: {panic:?}"))
        } else {
            Ok(())
        };
        cancel_result?;
        join_result?;
        tracing::info!("Worker gracefully stopped.");
        Ok(())
    }

    /// Prints all progress responses currently available and returns their count.
    pub fn process_responses(&self) -> usize {
        let mut processed = 0;
        while let Ok(task_response) = self.r_taskresponse.try_recv() {
            println!("Processed a response from task {}.", task_response.task_id);
            processed += 1;
        }
        processed
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.die();
    }
}
