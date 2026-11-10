//! Integration tests for the `cancelable` crate.
//!
//! Verifies worker lifecycle, cancellation behavior under high queue load,
//! and thread termination across independent process invocations.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;
use std::thread;
use std::time::Duration;

use cancelable::Task;
use cancelable::Worker;

/// Verifies worker initialization, task submission, response draining,
/// and graceful thread shutdown via both explicit termination and drop.
#[test]
fn test_worker_lifecycle() -> anyhow::Result<()> {
    // Test explicit shutdown lifecycle.
    let mut worker = Worker::new();
    let task = Task::new();
    let task_id = task.task_id();

    worker.start(task)?;

    // Give the task time to produce at least one progress response.
    thread::sleep(Duration::from_millis(150));

    let processed = worker.process_responses();
    assert!(
        processed >= 1,
        "worker should have processed at least one response"
    );

    // Cancel the running task and stop the worker.
    worker.cancel_task(task_id)?;
    worker.die()?;

    // Test drop cleanup lifecycle without explicit call to die().
    let worker_auto_drop = Worker::new();
    worker_auto_drop.start(Task::new())?;
    thread::sleep(Duration::from_millis(50));
    drop(worker_auto_drop);

    Ok(())
}

/// Verifies task submission up to maximum queue capacity, rejection of excess tasks,
/// targeted cancellation under saturation, and graceful drain and shutdown.
#[test]
fn test_cancellation_under_high_queue_load() -> anyhow::Result<()> {
    let mut worker = Worker::new();
    let mut task_ids = Vec::new();

    // The worker queue has a capacity of 5 tasks and each task runs for about 300 ms.
    // Submit many tasks in a tight loop: at most one running task plus a full queue can be
    // accepted, regardless of how quickly the worker thread starts, so the rest are rejected.
    let mut rejected = 0_usize;
    for _ in 0..20 {
        let task = Task::new();
        let task_id = task.task_id();
        if worker.start(task).is_ok() {
            task_ids.push(task_id);
        } else {
            rejected += 1;
        }
    }
    assert!(
        task_ids.len() <= 6,
        "at most the running task plus a full queue may be accepted, got {}",
        task_ids.len()
    );
    assert!(
        rejected >= 14,
        "task queue should reject submissions when full"
    );

    // Cancel an active or queued task.
    if let Some(&first_id) = task_ids.first() {
        let _ = worker.cancel_task(first_id);
    }

    // Allow worker to make progress.
    thread::sleep(Duration::from_millis(120));

    // Gracefully terminate the worker and interrupt remaining queued tasks.
    worker.die()?;

    let responses = worker.process_responses();
    assert!(
        responses > 0,
        "worker should have recorded progress responses before shutdown"
    );

    Ok(())
}

/// Verifies that independent process invocations of the binary cleanly start,
/// cancel background workers, and terminate all threads without hanging.
#[test]
fn test_thread_termination_across_independent_process_invocations() {
    let binary_path = env!("CARGO_BIN_EXE_cancelable");

    // Execute multiple independent process invocations in sequence.
    for iteration in 0..2 {
        let output = Command::new(binary_path).output().unwrap_or_else(|error| {
            panic!("failed to spawn binary on iteration {iteration}: {error}")
        });

        assert!(
            output.status.success(),
            "invocation {iteration} failed with status {:?}",
            output.status.code()
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Processed a response"),
            "invocation {iteration} should produce processed response output"
        );
    }
}
