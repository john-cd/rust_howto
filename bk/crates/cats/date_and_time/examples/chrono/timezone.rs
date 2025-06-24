#![allow(dead_code)]
// ANCHOR: example
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::TimeZone;

/// Demonstrates how to work with timezones in Chrono.
fn main() {
    // Get the current local time.
    let local_time = chrono::Local::now();
    // Separate the local time into its UTC representation and the offset.
    let utc_time = local_time.naive_utc();
    let offset = *local_time.offset();
    // Serialize, pass through FFI... and recreate the `DateTime`:
    let local_time_new =
        DateTime::<Local>::from_naive_utc_and_offset(utc_time, offset);
    assert_eq!(local_time, local_time_new);

    // Get the current UTC time.
    let _utc_time = chrono::Utc::now();

    // Define timezones for Hong Kong and Rio de Janeiro.
    let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let rio_timezone = FixedOffset::west_opt(2 * 3600).unwrap();
    // Print the time in different timezones.
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        china_timezone.from_utc_datetime(&utc_time)
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        rio_timezone.from_utc_datetime(&utc_time)
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
