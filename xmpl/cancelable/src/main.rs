//! Adapted from <https://dystroy.org/btracing::info!/cancelable-background-task/>.
//!
//! TODO Reviewed ??-??-2026.

use cancelable::*;

/// Runs the src command-line application.
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        // Sets the maximum verbosity level.
        .with_max_level(tracing::Level::DEBUG)
        // Sets this subscriber to be the global trace collector for this application.
        .init();

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
