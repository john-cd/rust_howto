//! Cancelling tasks with tokens that are checked while the task runs.
//!
//! TODO Reviewed ??-??-2026.

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that creating a token sets its initial cancellation state to uncancelled.
    #[test]
    fn test_make_token_and_check() -> anyhow::Result<()> {
        let source = CancelTokenSource::new();
        let token = source.make_token(42)?;
        assert_eq!(token.check(), Ok(()));
        Ok(())
    }

    /// Verifies that cancelling an active task causes `check()` to report an interruption error.
    #[test]
    fn test_cancel_task() -> anyhow::Result<()> {
        let source = CancelTokenSource::new();
        let token = source.make_token(10)?;
        source.cancel_task(10)?;
        assert_eq!(token.check(), Err(TaskError::Interrupted));
        Ok(())
    }

    /// Verifies that attempting to cancel a task ID that is not registered returns an error.
    #[test]
    fn test_cancel_task_not_active() {
        let source = CancelTokenSource::new();
        let result = source.cancel_task(99);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .is_some_and(|e| e.to_string().contains("is not active"))
        );
    }

    /// Verifies that `cancel_all` flags all currently registered tasks as cancelled.
    #[test]
    fn test_cancel_all() -> anyhow::Result<()> {
        let source = CancelTokenSource::new();
        let token1 = source.make_token(1)?;
        let token2 = source.make_token(2)?;
        let token3 = source.make_token(3)?;

        assert_eq!(token1.check(), Ok(()));
        assert_eq!(token2.check(), Ok(()));
        assert_eq!(token3.check(), Ok(()));

        source.cancel_all()?;

        assert_eq!(token1.check(), Err(TaskError::Interrupted));
        assert_eq!(token2.check(), Err(TaskError::Interrupted));
        assert_eq!(token3.check(), Err(TaskError::Interrupted));
        Ok(())
    }

    /// Verifies that attempting to register an ID that is already active fails.
    #[test]
    fn test_duplicate_registration_fails() -> anyhow::Result<()> {
        let source = CancelTokenSource::new();
        let _token = source.make_token(7)?;
        let duplicate_result = source.make_token(7);
        assert!(duplicate_result.is_err());
        assert!(
            duplicate_result
                .err()
                .is_some_and(|e| e.to_string().contains("already registered"))
        );
        Ok(())
    }

    /// Verifies that dropping a token removes the task registration, allowing the ID to be reused.
    #[test]
    fn test_token_drop_deregisters_task() -> anyhow::Result<()> {
        let source = CancelTokenSource::new();
        let token = source.make_token(15)?;
        assert!(source.make_token(15).is_err());
        drop(token);
        // Once dropped, registering the same task ID should succeed.
        let token_new = source.make_token(15)?;
        assert_eq!(token_new.check(), Ok(()));
        Ok(())
    }

    /// Verifies that `CancelTokenSource::default()` functions the same as `new()`.
    #[test]
    fn test_default_source() -> anyhow::Result<()> {
        let source = CancelTokenSource::default();
        let token = source.make_token(100)?;
        assert_eq!(token.check(), Ok(()));
        source.cancel_task(100)?;
        assert_eq!(token.check(), Err(TaskError::Interrupted));
        Ok(())
    }

    /// Verifies that dropping a token when the source has already been dropped does not panic.
    #[test]
    fn test_token_drop_after_source_dropped() -> anyhow::Result<()> {
        let token = {
            let source = CancelTokenSource::new();
            source.make_token(5)?
        };
        // Source is now dropped; dropping token should safely exit when weak pointer fails to upgrade.
        drop(token);
        Ok(())
    }

    /// Verifies that dropping a stale token does not remove an active task with the same ID.
    #[test]
    fn test_stale_token_drop_does_not_remove_active_task() -> anyhow::Result<()> {
        let source = CancelTokenSource::new();
        let token1 = source.make_token(20)?;

        // Manually craft a stale token pointing to the same registry and ID, but with a different AtomicBool.
        let stale_token = CancelToken {
            id: 20,
            cancelled: Arc::new(AtomicBool::new(false)),
            tasks: Arc::downgrade(&source.tasks),
        };

        // Dropping the stale token should not remove token1's registration because the Arc pointers differ.
        drop(stale_token);

        // Attempting to register ID 20 should still fail because token1 is still registered.
        assert!(source.make_token(20).is_err());

        // Now cancel task 20 and verify token1 reflects the cancellation.
        source.cancel_task(20)?;
        assert_eq!(token1.check(), Err(TaskError::Interrupted));

        // Dropping the genuine token1 removes the task.
        drop(token1);
        assert!(source.make_token(20).is_ok());
        Ok(())
    }
}
