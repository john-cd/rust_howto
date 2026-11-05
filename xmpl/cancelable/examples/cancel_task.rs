//! Starts a task on a worker thread, cancels it, then stops the worker.
//!
//! Run with `cargo run -p cancelable --example cancel_task`.
//!
//! TODO Reviewed ??-??-2026.

use std::thread;
use std::time::Duration;

use cancelable::Task;
use cancelable::Worker;

/// Runs the example.
fn main() -> anyhow::Result<()> {
    let mut worker = Worker::new();
    let task = Task::new();
    let id = task.task_id();
    worker.start(task)?;
    thread::sleep(Duration::from_millis(150));
    println!("processed {} responses", worker.process_responses());
    worker.cancel_task(id)?;
    worker.die()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example runs to completion.
    #[test]
    fn example_runs() {
        main().expect("example should succeed");
    }
}
