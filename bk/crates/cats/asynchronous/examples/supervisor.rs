#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Restarts a failing task with capped exponential backoff.

use std::time::Duration;

use tokio_util::sync::CancellationToken;

/// Runs `job` until it succeeds or `token` is cancelled.
///
/// Returns the number of attempts made and whether the job succeeded.
async fn supervise<F, Fut>(mut job: F, token: CancellationToken) -> (u32, bool)
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = Result<(), String>>,
{
    let mut delay = Duration::from_millis(10);
    let max_delay = Duration::from_millis(40);
    let mut attempts = 0;
    loop {
        attempts += 1;
        match job(attempts).await {
            Ok(()) => return (attempts, true),
            Err(e) => eprintln!("attempt {attempts} failed: {e}"),
        }
        tokio::select! {
            () = token.cancelled() => return (attempts, false),
            () = tokio::time::sleep(delay) => {}
        }
        delay = (delay * 2).min(max_delay);
    }
}

/// Fails until attempt `succeed_at`, then succeeds.
async fn flaky(attempt: u32, succeed_at: u32) -> Result<(), String> {
    if attempt < succeed_at {
        Err("not yet".to_owned())
    } else {
        Ok(())
    }
}

/// Runs the supervisor and prints its outcome.
async fn example() {
    let result = supervise(|n| flaky(n, 3), CancellationToken::new()).await;
    println!("(attempts, succeeded): {result:?}");
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
    use tokio::time::Instant;

    use super::*;

    /// Verifies the example entry point runs successfully.
    #[test]
    fn main_runs() -> anyhow::Result<()> {
        run()
    }

    /// Verifies the backoff delays double and are capped.
    #[tokio::test(start_paused = true)]
    async fn backoff_is_capped() {
        let start = Instant::now();
        let result = supervise(|n| flaky(n, 5), CancellationToken::new()).await;
        assert_eq!(result, (5, true));
        // 10 + 20 + 40 + 40 ms of backoff.
        assert_eq!(start.elapsed(), Duration::from_millis(110));
    }

    /// Verifies cancellation stops the restart loop.
    #[tokio::test(start_paused = true)]
    async fn cancellation_stops_supervisor() {
        let token = CancellationToken::new();
        token.cancel();
        let result = supervise(|n| flaky(n, u32::MAX), token).await;
        assert_eq!(result, (1, false));
    }
}
