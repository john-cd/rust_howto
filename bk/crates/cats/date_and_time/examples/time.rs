// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrate various functionalities of the `time` crate.
//!
//! The `time` crate is a lighter alternative to `chrono` for dates, times and
//! durations. UTC is the world reference time; local times are UTC plus an
//! offset that can change with daylight saving.

use time::Date;
use time::Duration;
use time::OffsetDateTime;
use time::PrimitiveDateTime;
use time::Time;
use time::format_description::well_known::Rfc3339;
use time::macros::datetime;
use time::macros::format_description;

/// Demonstrate various functionalities of the `time` crate.
///
/// # Errors
///
/// Returns an error if a date, format description, or formatted value is
/// invalid.
fn main() -> anyhow::Result<()> {
    // `OffsetDateTime::now_utc()` retrieves the current date and time
    // with the UTC offset.
    let now = OffsetDateTime::now_utc();
    println!("Current date and time (UTC): {now}");

    // Parse a date from a string.
    let date = Date::parse(
        "2025-01-02",
        &format_description!("[year]-[month]-[day]"),
    )?;
    println!("Parsed date: {date}");

    // Create a specific time using `Time::from_hms()`.
    let time = Time::from_hms(12, 30, 45)?;
    println!("Specific time: {time}");

    // Create a date and time using `PrimitiveDateTime::new()`.
    let datetime = PrimitiveDateTime::new(date, time);
    println!("Specific date and time: {datetime}");

    // Add a duration to a date and time.
    let later = datetime + Duration::hours(5);
    println!("Date and time after 5 hours: {later}");

    // Format a date and time as a string.
    let format =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let formatted = datetime!(2025-01-02 12:30 UTC).format(&format)?;
    println!("Formatted fixed date and time: {formatted}");
    let rfc3339 = OffsetDateTime::parse("2025-01-02T12:30:00Z", &Rfc3339)?;
    println!("Parsed RFC 3339 timestamp: {rfc3339}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the `time` crate example.
pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies fixed-date parsing, formatting, and arithmetic.
    #[test]
    fn time_example_parses_and_formats_values() -> anyhow::Result<()> {
        let fixed = datetime!(2025-01-02 12:30 UTC);
        let format = format_description!("[year]-[month]-[day]");
        assert_eq!(fixed.format(&format)?, "2025-01-02");
        assert_eq!(Date::parse("2025-01-02", &format)?, fixed.date());
        assert_eq!(fixed + Duration::hours(5), datetime!(2025-01-02 17:30 UTC));
        main()
    }
}
