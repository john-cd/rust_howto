//! Adapted from <https://dystroy.org/btracing::info!/cancelable-background-task/>.
//!
//! TODO Reviewed ??-??-2026.

use cancelable::*;

/// Runs the src command-line application.
fn main() -> anyhow::Result<()> {
    let _ignored = tracing_subscriber::fmt()
        // Sets the maximum verbosity level.
        .with_max_level(tracing::Level::DEBUG)
        // Sets this subscriber to be the global trace collector for this application.
        // `try_init` tolerates a collector that is already set (e.g. by another test).
        .try_init();

    let mut worker = Worker::new();
    let task = Task::new();
    let task_id = task.task_id();
    worker.start(task)?;
    std::thread::sleep(std::time::Duration::from_millis(250));
    worker.cancel_task(task_id)?;
    worker.die()?;
    worker.process_responses();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the demo runs to completion.
    #[test]
    fn main_runs() {
        main().unwrap();
    }

    /// Checks that cancelling a task after it has started stops it early.
    #[test]
    fn cancelled_task_sends_fewer_responses() {
        let mut worker = Worker::new();
        let task = Task::new();
        let id = task.task_id();
        worker.start(task).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50));
        worker.cancel_task(id).unwrap();
        worker.die().unwrap();
        // A full run reports 4 steps; cancellation stops it before that.
        assert!(worker.process_responses() < 4);
    }

    /// Checks that a task that is not active cannot be cancelled.
    #[test]
    fn cancel_unknown_task_fails() {
        let worker = Worker::new();
        let error = worker.cancel_task(u64::MAX).unwrap_err();
        assert!(error.to_string().contains("not active"));
    }

    /// Checks that identifiers are unique.
    #[test]
    fn task_ids_are_unique() {
        assert_ne!(Task::new().task_id(), Task::new().task_id());
    }

    /// Checks that a token is registered once and reports cancellation.
    #[test]
    fn cancel_token_lifecycle() {
        let source = CancelTokenSource::new();
        let token = source.make_token(1).unwrap();
        assert!(source.make_token(1).is_err());
        assert_eq!(token.check(), Ok(()));
        source.cancel_all().unwrap();
        assert_eq!(token.check(), Err(TaskError::Interrupted));
        drop(token);
        // The ID can be reused once the token is dropped.
        assert!(source.make_token(1).is_ok());
    }

    /// Checks that a full queue rejects new tasks.
    #[test]
    fn full_queue_rejects_tasks() {
        let worker = Worker::new();
        let mut results = Vec::new();
        for _ in 0..20 {
            results.push(worker.start(Task::new()));
        }
        assert!(results.iter().any(Result::is_err));
        assert!(results[0].is_ok());
    }
}
