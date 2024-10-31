use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd_opt(2017, 11, 12)
        .unwrap()
        .and_hms_opt(17, 33, 44)
        .unwrap();
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is
    {}.",
        date_time,
        date_time.and_utc().timestamp()
    );

    let date_time_after_a_billion_seconds =
        DateTime::from_timestamp(1_000_000_000, 0).unwrap();
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was
    {}.",
        date_time_after_a_billion_seconds
    );
}

#[test]
fn test() {
    main();
}
