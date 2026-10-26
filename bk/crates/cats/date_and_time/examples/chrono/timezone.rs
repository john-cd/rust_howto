#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates how to work with timezones in Chrono.

use chrono::FixedOffset;
use chrono::TimeZone as _;

/// Demonstrates how to work with timezones in Chrono.
fn main() {
    // Use a fixed UTC value for deterministic output and tests.
    let utc_time = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
        .and_then(|date| date.and_hms_opt(12, 0, 0))
        .expect("valid fixed date and time");
    let utc = chrono::Utc.from_utc_datetime(&utc_time);

    // Define two fixed UTC offsets.
    let plus_eight = FixedOffset::east_opt(8 * 3600).expect("valid UTC offset");
    let minus_two = FixedOffset::west_opt(2 * 3600).expect("valid UTC offset");
    // Print the time in different timezones.
    println!("UTC time is {utc}");
    println!(
        "Time at UTC+08:00 is {}",
        plus_eight.from_utc_datetime(&utc_time)
    );
    println!(
        "Time at UTC-02:00 is {}",
        minus_two.from_utc_datetime(&utc_time)
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use chrono::Timelike as _;

    use super::*;

    /// Verifies that fixed offsets produce stable converted clock times.
    #[test]
    fn fixed_utc_time_converts_to_expected_offsets() {
        let utc_time = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
            .and_then(|date| date.and_hms_opt(12, 0, 0))
            .expect("valid fixed date and time");
        let hong_kong = FixedOffset::east_opt(8 * 3600)
            .expect("valid offset")
            .from_utc_datetime(&utc_time);
        assert_eq!(hong_kong.hour(), 20);
        assert_eq!(hong_kong.date_naive().to_string(), "2025-01-15");
        main();
    }
}
