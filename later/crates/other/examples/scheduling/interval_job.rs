// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A periodic job with `tokio::time::interval`.
//!
//! `MissedTickBehavior::Skip` drops most of the ticks missed while a run was
//! still in progress, instead of firing a burst to catch up. The loop ends when
//! the cancellation token fires. The test runtime has a paused clock, so no
//! real time passes.

use std::time::Duration;

use tokio::select;
use tokio::time::MissedTickBehavior;
use tokio::time::interval;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// Runs `job` every `period` until `token` is cancelled. Returns how many
/// times the job ran.
async fn run_periodically<F, Fut>(
    period: Duration,
    token: CancellationToken,
    mut job: F,
) -> u32
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let mut ticker = interval(period);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut runs = 0;
    loop {
        select! {
            biased;
            () = token.cancelled() => return runs,
            _ = ticker.tick() => {
                job().await;
                runs += 1;
            }
        }
    }
}

/// Runs a job every 10 s for 35 s: ticks at 0, 10, 20 and 30 s.
async fn demo() -> anyhow::Result<()> {
    let token = CancellationToken::new();
    let stopper = token.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(35)).await;
        stopper.cancel();
    });
    let runs =
        run_periodically(Duration::from_secs(10), token, || async {}).await;
    assert_eq!(runs, 4);

    // A job slower than the period: runs of 25 s with a 10 s period.
    let token = CancellationToken::new();
    let stopper = token.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(124)).await;
        stopper.cancel();
    });
    let origin = tokio::time::Instant::now();
    let starts = std::sync::Mutex::new(Vec::new());
    let runs = run_periodically(Duration::from_secs(10), token, || async {
        if let Ok(mut starts) = starts.lock() {
            starts.push(origin.elapsed().as_secs());
        }
        sleep(Duration::from_secs(25)).await;
    })
    .await;
    // A missed tick fires as soon as the run ends; subsequent missed ticks
    // are skipped. The cancellation becomes ready during the fifth run, and
    // cancellation has priority over another tick after that run ends. With
    // `MissedTickBehavior::Delay` the next tick is instead one full period
    // after the late one.
    assert_eq!(runs, 5);
    assert_eq!(
        starts.lock().map(|s| s.clone()).ok(),
        Some(vec![0, 25, 50, 75, 100])
    );
    Ok(())
}

/// Runs the demonstration on a paused-clock runtime and reports setup errors.
fn main() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    runtime.block_on(demo())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exercises the interval and missed-tick behavior using Tokio's paused
    /// clock.
    #[tokio::test(start_paused = true)]
    async fn paused_clock_runs_expected_ticks() -> anyhow::Result<()> {
        demo().await
    }
}
