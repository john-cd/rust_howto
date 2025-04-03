// ANCHOR: example
use time::Date;
use time::Duration;
use time::OffsetDateTime;
use time::PrimitiveDateTime;
use time::Time;
use time::format_description;

/// Demonstrate various functionalities of the `time` crate.
fn main() {
    // `OffsetDateTime::now_utc()` retrieves the current date and time
    // with the UTC offset.
    let now = OffsetDateTime::now_utc();
    println!("Current date and time (UTC): {}", now);

    // Parse a date from a string.
    let date = Date::parse(
        "2025-01-02",
        &format_description::parse("[year]-[month]-[day]").unwrap(),
    )
    .unwrap();
    println!("Parsed date: {}", date);

    // Create a specific time using `Time::from_hms()`.
    let time = Time::from_hms(12, 30, 45).unwrap();
    println!("Specific time: {}", time);

    // Create a date and time using `PrimitiveDateTime::new()`.
    let datetime = PrimitiveDateTime::new(date, time);
    println!("Specific date and time: {}", datetime);

    // Add a duration to a date and time.
    let later = datetime + Duration::hours(5);
    println!("Date and time after 5 hours: {}", later);

    // Format a date and time as a string.
    let formatted = now
        .format(
            &format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second]",
            )
            .unwrap(),
        )
        .unwrap();
    println!("Formatted date and time: {}", formatted);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
