// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Cancels a tree of workers with parent and child `CancellationToken`s.

use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// Ticks until `token` is cancelled, then returns how many ticks it saw.
async fn worker(token: CancellationToken) -> u32 {
    let mut interval = tokio::time::interval(Duration::from_millis(10));
    let mut ticks = 0;
    loop {
        tokio::select! {
            () = token.cancelled() => return ticks,
            _ = interval.tick() => ticks += 1,
        }
    }
}

/// Cancels one child, then the parent, and reports the final token states.
async fn cancellation_states() -> (bool, bool, bool) {
    let parent = CancellationToken::new();
    let child_a = parent.child_token();
    let child_b = parent.child_token();

    let tracker = TaskTracker::new();
    let a = tracker.spawn(worker(child_a.clone()));
    let b = tracker.spawn(worker(child_b.clone()));
    tracker.close();

    // Cancelling a child does not affect the parent or its siblings.
    child_a.cancel();
    let _ = a.await;
    assert!(!parent.is_cancelled());
    assert!(!child_b.is_cancelled());

    // Cancelling the parent cancels every remaining child.
    parent.cancel();
    let _ = b.await;
    // `wait` returns once the tracker is closed and every task has finished.
    tracker.wait().await;
    (
        parent.is_cancelled(),
        child_a.is_cancelled(),
        child_b.is_cancelled(),
    )
}

/// Cancels the worker tree and prints its final token states.
async fn example() {
    let states = cancellation_states().await;
    println!("parent, child_a, child_b cancelled: {states:?}");
}

/// Runs the example on a current-thread Tokio runtime.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    example().await;
}

// ANCHOR_END: example

/// Runs the example on a current-thread Tokio runtime.
///
/// # Errors
///
/// Returns an error if the Tokio runtime cannot be created.
pub fn run() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(example());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point runs successfully.
    #[test]
    fn main_runs() -> anyhow::Result<()> {
        run()
    }

    /// Verifies cancellation flows down the tree and not up.
    #[tokio::test(start_paused = true)]
    async fn parent_cancels_children_not_the_reverse() {
        assert_eq!(cancellation_states().await, (true, true, true));
    }

    /// Verifies a worker stops promptly when cancelled.
    #[tokio::test(start_paused = true)]
    async fn worker_stops_on_cancel() {
        let token = CancellationToken::new();
        let handle = tokio::spawn(worker(token.clone()));
        tokio::time::sleep(Duration::from_millis(35)).await;
        token.cancel();
        let ticks = handle.await.expect("worker must not panic");
        assert!(ticks >= 3);
    }
}
