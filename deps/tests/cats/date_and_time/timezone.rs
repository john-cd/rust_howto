use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::TimeZone;

fn main() {
    let local_time = chrono::Local::now();
    // Separate into components
    let utc_time = local_time.naive_utc();
    let offset = *local_time.offset();
    // Serialize, pass through FFI... and recreate the `DateTime`:
    let local_time_new =
        DateTime::<Local>::from_naive_utc_and_offset(utc_time, offset);
    assert_eq!(local_time, local_time_new);

    // there is also
    let _utc_time = chrono::Utc::now();

    let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let rio_timezone = FixedOffset::west_opt(2 * 3600).unwrap();
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

#[test]
fn test() {
    main();
}
