// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to get the current UTC time and date using the chrono
//! crate. It prints the time in 12-hour format with AM/PM, seconds since
//! midnight, and the date in year-month-day format.

use chrono::Datelike as _;
use chrono::Timelike as _;
use chrono::Utc;

/// Demonstrates how to get the current UTC time and date using the chrono
/// crate. It prints the time in 12-hour format with AM/PM, seconds since
/// midnight, and the date in year-month-day format.
fn main() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {hour:02}:{:02}:{:02} {}",
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {year}-{:02}-{:02} {:?} ({})",
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the current example through its entry point.
    #[test]
    fn test_current_example_entry_point() {
        main();
    }

    #[test]
    fn current_time_components_are_consistent() {
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        assert!((1..=12).contains(&hour));
        assert_eq!(is_pm, now.hour() >= 12);
        assert_eq!(
            now.num_seconds_from_midnight(),
            now.hour() * 3600 + now.minute() * 60 + now.second()
        );
        let (is_common_era, year) = now.year_ce();
        assert!(is_common_era);
        assert!(year >= 2026);
        assert!(now.num_days_from_ce() > 739_000);
    }

    #[test]
    fn hour12_maps_midnight_and_noon() {
        let midnight = chrono::NaiveTime::from_hms_opt(0, 0, 0)
            .expect("midnight is a valid time of day");
        let noon = chrono::NaiveTime::from_hms_opt(12, 0, 0)
            .expect("noon is a valid time of day");
        assert_eq!(midnight.hour12(), (false, 12));
        assert_eq!(noon.hour12(), (true, 12));
    }
}
