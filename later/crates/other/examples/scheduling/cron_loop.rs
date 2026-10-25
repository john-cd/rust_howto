#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Sleep until the next cron time, run the job, repeat: no drift, because each
//! wake-up is computed from the schedule rather than from "now plus a period".
//!
//! If a run takes longer than the gap to the next fire time, the next time is
//! computed *after* the job finishes, so the missed fire times are skipped
//! rather than run back to back. (Compute from the previous fire time instead
//! if you want to catch up.)

use std::str::FromStr;
use std::time::Duration;

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use cron::Schedule;
use tokio::select;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// A clock that follows Tokio's (pausable) time: `start` plus elapsed time.
struct Clock {
    start: DateTime<Utc>,
    origin: tokio::time::Instant,
}

impl Clock {
    fn new(start: DateTime<Utc>) -> Self {
        Self {
            start,
            origin: tokio::time::Instant::now(),
        }
    }

    fn now(&self) -> DateTime<Utc> {
        self.start + self.origin.elapsed()
    }
}

/// Runs `job` at every fire time of `schedule` until cancelled. Returns the
/// fire times at which the job started.
async fn run_cron<F, Fut>(
    schedule: &Schedule,
    clock: &Clock,
    token: CancellationToken,
    mut job: F,
) -> Vec<DateTime<Utc>>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let mut started = Vec::new();
    loop {
        let Some(next) = schedule.after(&clock.now()).next() else {
            return started; // The schedule has ended.
        };
        let wait = (next - clock.now()).to_std().unwrap_or(Duration::ZERO);
        select! {
            () = token.cancelled() => return started,
            () = sleep(wait) => {}
        }
        started.push(next);
        job().await;
    }
}

/// Runs a "every 10 seconds" schedule, with a job that sometimes overruns.
async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    let schedule = Schedule::from_str("*/10 * * * * *")?;
    let start = Utc
        .with_ymd_and_hms(2026, 10, 25, 12, 0, 0)
        .single()
        .ok_or("invalid date")?;
    let clock = Clock::new(start);

    let token = CancellationToken::new();
    let stopper = token.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(65)).await;
        stopper.cancel();
    });

    // The first run takes 25 s (it overruns two fire times); the others are
    // instantaneous.
    let mut first = true;
    let started = run_cron(&schedule, &clock, token, || {
        let slow = std::mem::take(&mut first);
        async move {
            if slow {
                sleep(Duration::from_secs(25)).await;
            }
        }
    })
    .await;

    let offsets: Vec<i64> =
        started.iter().map(|t| (*t - start).num_seconds()).collect();
    // Starts at 10 s; the run ends at 35 s; the fire times at 20 s and 30 s
    // were skipped; then 40, 50 and 60 s.
    assert_eq!(offsets, vec![10, 40, 50, 60]);
    Ok(())
}

/// Runs the demonstration on a paused-clock runtime.
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
    if let Err(e) = runtime.block_on(demo()) {
        eprintln!("{e}");
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
