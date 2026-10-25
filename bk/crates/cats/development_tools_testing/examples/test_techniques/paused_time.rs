#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Testing timeouts and retries instantly with Tokio's paused clock.

use std::time::Duration;

use tokio::time::sleep;
use tokio::time::timeout;

/// Waits `delay`, then returns the answer.
async fn slow_answer(delay: Duration) -> u32 {
    sleep(delay).await;
    42
}

#[cfg(test)]
mod tests {
    use tokio::time::Instant;

    use super::*;

    // With `start_paused`, the runtime advances the clock automatically
    // whenever every task is waiting on a timer, so the test does not really
    // sleep.
    #[tokio::test(start_paused = true)]
    async fn times_out_without_waiting() {
        let started = Instant::now();
        let result = timeout(
            Duration::from_secs(5),
            slow_answer(Duration::from_secs(60)),
        )
        .await;
        assert!(result.is_err());
        // Virtual time moved by the timeout, not by the 60 seconds.
        assert_eq!(started.elapsed(), Duration::from_secs(5));
    }

    #[tokio::test(start_paused = true)]
    async fn completes_in_time() {
        let result = timeout(
            Duration::from_secs(5),
            slow_answer(Duration::from_secs(1)),
        )
        .await;
        assert_eq!(result, Ok(42));
    }
}
// ANCHOR_END: example

/// The example runs under `cargo test`.
fn main() {}
