// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Testing timeouts and retries instantly with Tokio's paused clock.
//!
//! Tokio can pause its clock in tests and advance it by hand, so code with
//! timeouts, retries or delays is tested instantly and deterministically
//! instead of with real sleeps.

use std::time::Duration;

use tokio::time::sleep;

/// Waits `delay`, then returns the answer.
async fn slow_answer(delay: Duration) -> u32 {
    sleep(delay).await;
    42
}
// ANCHOR_END: example

/// Waits briefly in real time; the tests below use a paused clock instead.
fn main() -> std::io::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    let answer = runtime.block_on(slow_answer(Duration::from_millis(10)));
    println!("The answer is {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio::time::Instant;
    use tokio::time::timeout;

    use super::*;

    // With `start_paused`, the runtime advances the clock automatically
    // whenever every task is waiting on a timer, so the test does not really
    // sleep.
    /// Advances virtual time to the timeout without sleeping in real time.
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

    /// Completes before the deadline on the paused Tokio clock.
    #[tokio::test(start_paused = true)]
    async fn completes_in_time() {
        let result = timeout(
            Duration::from_secs(5),
            slow_answer(Duration::from_secs(1)),
        )
        .await;
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_main_entry_point() -> std::io::Result<()> {
        main()
    }
}
