#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates checked arithmetic with `DateTime` and `Duration`.

use chrono::DateTime;
use chrono::Duration;
use chrono::TimeDelta;
use chrono::TimeZone as _;
use chrono::Utc;

/// Returns the date and time one day earlier than the given date and time, or
/// None if the operation would overflow.
fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(
        Duration::try_days(1).expect("one day fits in a chrono duration"),
    )
}

/// Demonstrates checked arithmetic with `DateTime` and `Duration`.
fn main() {
    let now = Utc
        .with_ymd_and_hms(2025, 1, 15, 12, 0, 0)
        .single()
        .expect("valid fixed date");

    let almost_three_weeks_from_now = now
        .checked_add_signed(
            Duration::try_weeks(2).expect("two weeks fit in a chrono duration"),
        )
        .and_then(|in_2weeks| {
            in_2weeks.checked_add_signed(
                Duration::try_weeks(1)
                    .expect("one week fits in a chrono duration"),
            )
        })
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{x}"),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(TimeDelta::MAX) {
        Some(x) => println!("{x}"),
        None => eprintln!(
            "We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."
        ),
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_arithmetic_handles_fixed_date_and_overflow() {
        let fixed = Utc
            .with_ymd_and_hms(2025, 1, 15, 12, 0, 0)
            .single()
            .expect("valid fixed date");
        assert!(day_earlier(fixed).is_some());
        assert!(fixed.checked_add_signed(TimeDelta::MAX).is_none());
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
