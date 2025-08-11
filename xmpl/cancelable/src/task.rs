use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crossbeam::channel::Sender;

use super::cancel::CancelToken;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct TaskResponse {
    pub task_id: u64,
}

type FinalTaskResponse = TaskResponse;

/// The task to execute.
#[derive(Debug, Clone)]
pub struct Task {
    task_id: u64,
}

impl Task {
    pub fn new() -> Self {
        let previous_value = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            task_id: previous_value,
        }
    }

    pub fn task_id(&self) -> u64 {
        self.task_id
    }

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
