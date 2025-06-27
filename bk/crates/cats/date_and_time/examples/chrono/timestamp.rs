#![allow(dead_code)]
// ANCHOR: example
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

fn main() {
    // Create a `NaiveDateTime` object representing November 12, 2017, at
    // 17:33:44.
    let date_time: NaiveDateTime = NaiveDate::from_ymd_opt(2017, 11, 12)
        // Unwrap the result of from_ymd_opt, which returns an Option.
        .unwrap()
        .and_hms_opt(17, 33, 44)
        .unwrap();
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is
    {}.",
        date_time,
        date_time.and_utc().timestamp()
    );

    // Create a `DateTime` object representing the time one billion seconds
    // after the Unix epoch.
    let date_time_after_a_billion_seconds =
        // from_timestamp returns an Option, so we unwrap it.
        DateTime::from_timestamp(1_000_000_000, 0).unwrap();
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {date_time_after_a_billion_seconds}."
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
