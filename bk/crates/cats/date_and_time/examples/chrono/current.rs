#![allow(dead_code)]
// ANCHOR: example
use chrono::Datelike;
use chrono::Timelike;
use chrono::Utc;

/// Demonstrates how to get the current UTC time and date using the chrono
/// crate. It prints the time in 12-hour format with AM/PM, seconds since
/// midnight, and the date in year-month-day format.
fn main() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
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
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
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

#[test]
fn test() {
    main();
}
