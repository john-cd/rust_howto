// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A bounded worker pool with `JoinSet`: keep at most `LIMIT` tasks running,
//! refill as tasks finish, collect results in completion order, and tell a
//! panic from a cancellation.
//!
//! The runtime is built with `start_paused(true)`: Tokio's clock only advances
//! when every task is waiting, so the sleeps below take no real time and the
//! completion order is deterministic.

use std::time::Duration;

use tokio::task::JoinSet;
use tokio::time::sleep;

/// The maximum number of tasks running at once.
const LIMIT: usize = 2;

/// What happened to the tasks.
#[derive(Debug, Default, PartialEq)]
struct Outcome {
    /// Results, in the order the tasks finished.
    results: Vec<u64>,
    panicked: usize,
    cancelled: usize,
}

/// One unit of work: the delay is the input; the result is its square.
async fn work(n: u64) -> u64 {
    sleep(Duration::from_millis(n)).await;
    // Simulate a bug.
    assert_ne!(n, 13, "unlucky input");
    n * n
}

/// Runs `inputs` with at most `LIMIT` tasks at a time.
async fn run_pool(inputs: Vec<u64>) -> Outcome {
    let mut pending = inputs.into_iter();
    let mut set = JoinSet::new();
    let mut outcome = Outcome::default();

    loop {
        // Fill the pool.
        while set.len() < LIMIT {
            match pending.next() {
                Some(n) => {
                    set.spawn(work(n));
                }
                None => break,
            }
        }
        // Wait for any one task; `None` means the set is empty (and, since we
        // just filled it, that there is no more input).
        match set.join_next().await {
            None => break,
            Some(Ok(value)) => outcome.results.push(value),
            Some(Err(e)) if e.is_panic() => outcome.panicked += 1,
            Some(Err(_)) => outcome.cancelled += 1,
        }
    }
    outcome
}

/// `abort_all` cancels the remaining tasks; they report a cancelled
/// `JoinError`.
async fn abort_demo() -> usize {
    let mut set = JoinSet::new();
    for _ in 0..3 {
        set.spawn(sleep(Duration::from_hours(1)));
    }
    set.abort_all();
    let mut cancelled = 0;
    while let Some(result) = set.join_next().await {
        if matches!(result, Err(ref e) if e.is_cancelled()) {
            cancelled += 1;
        }
    }
    cancelled
}

/// Runs the pool and the abort demonstration on a paused-clock runtime.
fn main() {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()
    {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("Cannot build the runtime: {e}");
            return;
        }
    };

    let outcome = runtime.block_on(run_pool(vec![30, 10, 13, 20, 5]));
    // 30 and 10 start; 10 finishes first (t=10), 13 starts and panics at t=13,
    // 20 starts at t=13 and ends at t=33; 30 ends at t=30; 5 starts at t=30.
    assert_eq!(outcome.results, vec![100, 900, 25, 400]);
    assert_eq!(outcome.panicked, 1);
    assert_eq!(outcome.cancelled, 0);

    assert_eq!(runtime.block_on(abort_demo()), 3);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the bounded pool returns each completed task result.
    /// Runs the joinset pool example through its entry point.
    #[test]
    fn test_joinset_pool_example_entry_point() {
        main();
    }
}
