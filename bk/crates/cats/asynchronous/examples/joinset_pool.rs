#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Runs a bounded pool of workers with `JoinSet`.

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;

use tokio::task::JoinSet;

/// Squares `n` after a delay, tracking how many workers run at once.
async fn work(n: u64, in_flight: Arc<AtomicUsize>, peak: Arc<AtomicUsize>) -> u64 {
    let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
    peak.fetch_max(now, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(10 * (10 - n % 10))).await;
    in_flight.fetch_sub(1, Ordering::SeqCst);
    n * n
}

/// Runs at most `limit` workers at a time and returns their results in
/// completion order, along with the highest observed concurrency.
async fn run_pool(inputs: Vec<u64>, limit: usize) -> (Vec<u64>, usize) {
    let in_flight = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let mut inputs = inputs.into_iter();
    let mut set = JoinSet::new();
    let mut results = Vec::new();

    // Fill the pool.
    for n in inputs.by_ref().take(limit) {
        set.spawn(work(n, in_flight.clone(), peak.clone()));
    }
    // Each time a task finishes, start the next one.
    while let Some(joined) = set.join_next().await {
        match joined {
            Ok(value) => results.push(value),
            Err(e) if e.is_panic() => eprintln!("a worker panicked: {e}"),
            Err(e) => eprintln!("a worker was cancelled: {e}"),
        }
        if let Some(n) = inputs.next() {
            set.spawn(work(n, in_flight.clone(), peak.clone()));
        }
    }
    (results, peak.load(Ordering::SeqCst))
}

/// Shows how a panic and an abort surface as a `JoinError`.
async fn join_errors() -> (bool, bool) {
    let mut set: JoinSet<()> = JoinSet::new();
    set.spawn(async { panic!("boom") });
    let panicked = set
        .join_next()
        .await
        .is_some_and(|r| r.is_err_and(|e| e.is_panic()));

    set.spawn(std::future::pending());
    set.abort_all();
    let cancelled = set
        .join_next()
        .await
        .is_some_and(|r| r.is_err_and(|e| e.is_cancelled()));
    (panicked, cancelled)
}

/// Runs the example.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (results, peak) = run_pool((1..=8).collect(), 3).await;
    println!("results (completion order): {results:?}, peak concurrency: {peak}");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies concurrency is bounded and every result is returned.
    #[tokio::test(start_paused = true)]
    async fn pool_is_bounded() {
        let (mut results, peak) = run_pool((1..=8).collect(), 3).await;
        assert!(peak <= 3);
        results.sort_unstable();
        assert_eq!(results, vec![1, 4, 9, 16, 25, 36, 49, 64]);
    }

    /// Verifies panics and aborts are reported as `JoinError`s.
    #[tokio::test]
    async fn join_errors_are_distinguished() {
        assert_eq!(join_errors().await, (true, true));
    }
}
