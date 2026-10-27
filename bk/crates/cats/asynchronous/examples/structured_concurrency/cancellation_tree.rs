#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A tree of `CancellationToken`s: cancelling a parent cancels its children,
//! but not the other way around. Workers check the token in a `select!`.
//!
//! Cancellation is cooperative: a task stops only at an `.await` where it
//! observes the token. Dropping a future cancels it immediately (at its next
//! suspension point, without running the rest of its body), and
//! `JoinHandle::abort` does the same from outside. A token lets the task clean
//! up first. `TaskTracker` waits for a group of tasks to finish.

use std::time::Duration;

use tokio::select;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// A worker that "works" for an hour unless it is cancelled.
async fn worker(token: CancellationToken) -> &'static str {
    select! {
        () = token.cancelled() => "cancelled",
        () = sleep(Duration::from_secs(3600)) => "finished",
    }
}

/// Cancelling the parent stops the children; the reverse is not true.
async fn demo() {
    let parent = CancellationToken::new();
    let child_a = parent.child_token();
    let child_b = parent.child_token();

    // Cancelling a child does not touch the parent or the sibling.
    child_a.cancel();
    assert!(child_a.is_cancelled());
    assert!(!parent.is_cancelled());
    assert!(!child_b.is_cancelled());

    // Clones share the same token.
    let clone = child_b.clone();
    let tracker = TaskTracker::new();
    let handle = tracker.spawn(worker(clone));
    let handle_a = tracker.spawn(worker(child_a));
    // No more tasks will be added; `wait` can complete.
    tracker.close();

    // Cancelling the parent reaches `child_b`.
    parent.cancel();
    tracker.wait().await;

    assert!(child_b.is_cancelled());
    assert_eq!(handle.await.ok(), Some("cancelled"));
    assert_eq!(handle_a.await.ok(), Some("cancelled"));
}

/// Runs the demonstration on a paused-clock runtime.
fn main() {
    match tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()
    {
        Ok(runtime) => runtime.block_on(demo()),
        Err(e) => eprintln!("Cannot build the runtime: {e}"),
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that cancelling the root token reaches all child tasks.
    /// Runs the cancellation tree example through its entry point.
    #[test]
    fn test_cancellation_tree_example_entry_point() {
        main();
    }
}
