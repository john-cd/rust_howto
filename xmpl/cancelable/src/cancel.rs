//! Cancelling tasks with tokens that are checked while the task runs.
//!
//! TODO Reviewed ??-??-2026

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use super::errors::TaskError;

/// Sends cancellation requests and creates one token for each active task.
#[derive(Debug, Clone)]
pub struct CancelTokenSource {
    /// Active task flags, shared with their tokens.
    tasks: Arc<Mutex<BTreeMap<u64, Arc<AtomicBool>>>>,
}

impl CancelTokenSource {
    /// Creates a source with no active tasks.
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    /// Registers a task and creates the token it checks while running.
    ///
    /// # Errors
    ///
    /// Returns an error if this ID is already active or the registry lock is poisoned.
    pub fn make_token(&self, id: u64) -> anyhow::Result<CancelToken> {
        let cancelled = Arc::new(AtomicBool::new(false));
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|error| anyhow::anyhow!("task cancellation registry is poisoned: {error}"))?;
        if tasks.contains_key(&id) {
            anyhow::bail!("task {id} is already registered for cancellation");
        }
        tasks.insert(id, Arc::clone(&cancelled));
        drop(tasks);
        Ok(CancelToken {
            id,
            cancelled,
            tasks: Arc::downgrade(&self.tasks),
        })
    }

    /// Requests cancellation of every registered task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task registry lock is poisoned.
    pub fn cancel_all(&self) -> anyhow::Result<()> {
        let tasks = self
            .tasks
            .lock()
            .map_err(|error| anyhow::anyhow!("task cancellation registry is poisoned: {error}"))?;
        for cancelled in tasks.values() {
            cancelled.store(true, Ordering::Release);
        }
        drop(tasks);
        tracing::info!("Cancellation requested for all active tasks.");
        Ok(())
    }

    /// Requests cancellation of the active task with the given ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the task ID is not active or the registry lock is poisoned.
    pub fn cancel_task(&self, id: u64) -> anyhow::Result<()> {
        let tasks = self
            .tasks
            .lock()
            .map_err(|error| anyhow::anyhow!("task cancellation registry is poisoned: {error}"))?;
        let cancelled = tasks
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("task {id} is not active"))?;
        cancelled.store(true, Ordering::Release);
        drop(tasks);
        tracing::info!("Cancellation requested for task {id}.");
        Ok(())
    }
}

impl Default for CancelTokenSource {
    fn default() -> Self {
        Self::new()
    }
}

/// A task's cooperative signal to stop running.
pub struct CancelToken {
    /// Identifier of the task.
    id: u64,
    /// The task's cancellation flag.
    cancelled: Arc<AtomicBool>,
    /// Registry used to remove the task when this token is dropped.
    tasks: Weak<Mutex<BTreeMap<u64, Arc<AtomicBool>>>>,
}

impl CancelToken {
    /// Returns an interruption error after cancellation has been requested.
    pub fn check(&self) -> Result<(), TaskError> {
        if self.cancelled.load(Ordering::Acquire) {
            Err(TaskError::Interrupted)
        } else {
            Ok(())
        }
    }
}

impl Drop for CancelToken {
    fn drop(&mut self) {
        let Some(tasks) = self.tasks.upgrade() else {
            return;
        };
        let mut tasks = match tasks.lock() {
            Ok(tasks) => tasks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if tasks
            .get(&self.id)
            .is_some_and(|cancelled| Arc::ptr_eq(cancelled, &self.cancelled))
        {
            tasks.remove(&self.id);
        }
    }
}
