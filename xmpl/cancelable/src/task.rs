//! A task that reports its progress and can be interrupted.
//!
//! TODO Reviewed ??-??-2026

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crossbeam::channel::Sender;

use super::cancel::CancelToken;

/// Counter that gives each task a unique identifier.
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// A progress report sent by a task.
#[derive(Debug)]
pub struct TaskResponse {
    /// Identifier of the task.
    pub task_id: u64,
}

/// The response sent when a task completes.
type FinalTaskResponse = TaskResponse;

/// The task to execute.
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique identifier of the task.
    task_id: u64,
}

impl Task {
    /// Creates a task with a new identifier.
    pub fn new() -> Self {
        let previous_value = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            task_id: previous_value,
        }
    }

    /// Returns the identifier of the task.
    pub fn task_id(&self) -> u64 {
        self.task_id
    }

    /// Runs the task, sending progress and stopping if `cancel_token` says so.
    pub(super) fn execute(
        self,
        cancel_token: CancelToken,
        taskresponse: &Sender<TaskResponse>,
    ) -> anyhow::Result<FinalTaskResponse> {
        for i in 1..=4 {
            cancel_token.check()?;
            tracing::info!("Task {}: {i}/4", self.task_id);
            taskresponse.send(TaskResponse {
                task_id: self.task_id,
            })?;
        }
        Ok(FinalTaskResponse {
            task_id: self.task_id,
        })
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new()
    }
}
