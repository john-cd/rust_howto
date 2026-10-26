#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Compute the next fire times of a cron expression with the `cron` crate.
//!
//! `cron` uses 6 or 7 fields: `sec min hour day-of-month month day-of-week
//! [year]`. Traditional 5-field expressions (no seconds) must be normalized
//! first, for example by prefixing `0 `. The schedule is evaluated in the time
//! zone of the `DateTime` you pass in; use `chrono-tz` for a named zone.

use std::str::FromStr;

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use cron::Schedule;

/// Turns a 5-field expression into the 6-field form `cron` expects.
fn normalize(expression: &str) -> String {
    if expression.split_whitespace().count() == 5 {
        format!("0 {expression}")
    } else {
        expression.to_owned()
    }
}

/// Returns the next `n` fire times after `after`.
fn next_times(
    expression: &str,
    after: DateTime<Utc>,
    n: usize,
) -> anyhow::Result<Vec<DateTime<Utc>>> {
    let schedule = Schedule::from_str(&normalize(expression))?;
    Ok(schedule.after(&after).take(n).collect())
}

/// Lists the next three runs of "09:30 on weekdays" after a fixed instant.
fn main() -> anyhow::Result<()> {
    // Friday 2026-10-23, 10:00 UTC: today's 09:30 has passed.
    let now = Utc
        .with_ymd_and_hms(2026, 10, 23, 10, 0, 0)
        .single()
        .ok_or_else(|| anyhow::anyhow!("invalid date"))?;

    let times = next_times("0 30 9 * * Mon-Fri *", now, 3)?;
    let expected: Vec<_> = [(2026, 10, 26), (2026, 10, 27), (2026, 10, 28)]
        .into_iter()
        .filter_map(|(y, m, d)| {
            Utc.with_ymd_and_hms(y, m, d, 9, 30, 0).single()
        })
        .collect();
    assert_eq!(times, expected);

    // The 5-field form gives the same schedule.
    assert_eq!(next_times("30 9 * * Mon-Fri", now, 3)?, expected);

    // Invalid expressions are an error, not a panic.
    assert!(next_times("not a cron expression", now, 1).is_err());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies deterministic next-fire times and invalid expression handling.
    #[test]
    fn computes_expected_weekday_fire_times() -> anyhow::Result<()> {
        main()
    }
}
