#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A supervisor that restarts a failing task with capped exponential backoff
//! and stops when it is cancelled.

use std::time::Duration;

use tokio::select;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// The delay before restart number `attempt` (starting at 0): `base *
/// 2^attempt`, capped at `cap`.
fn backoff(attempt: u32, base: Duration, cap: Duration) -> Duration {
    base.saturating_mul(2_u32.saturating_pow(attempt)).min(cap)
}

/// Runs `task` until it succeeds or `token` is cancelled. Returns the delays
/// that were waited between attempts.
async fn supervise<F, Fut>(
    token: CancellationToken,
    mut task: F,
) -> Vec<Duration>
where
    F: FnMut(u32) -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    let mut delays = Vec::new();
    let mut attempt = 0;
    loop {
        match task(attempt).await {
            Ok(()) => return delays,
            Err(e) => eprintln!("Attempt {attempt} failed: {e}"),
        }
        let delay = backoff(
            attempt,
            Duration::from_millis(100),
            Duration::from_millis(400),
        );
        select! {
            () = token.cancelled() => return delays,
            () = sleep(delay) => {}
        }
        delays.push(delay);
        attempt += 1;
    }
}

/// Fails four times, then succeeds.
async fn demo() {
    let token = CancellationToken::new();
    let delays = supervise(token, |attempt| async move {
        if attempt < 4 {
            Err(format!("simulated failure {attempt}"))
        } else {
            Ok(())
        }
    })
    .await;
    // 100, 200, 400, then capped at 400 ms.
    let millis: Vec<u128> = delays.iter().map(Duration::as_millis).collect();
    assert_eq!(millis, vec![100, 200, 400, 400]);

    // A cancelled supervisor stops instead of restarting.
    let token = CancellationToken::new();
    token.cancel();
    let delays =
        supervise(token, |_| async { Err("always fails".to_owned()) }).await;
    assert!(delays.is_empty());
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
