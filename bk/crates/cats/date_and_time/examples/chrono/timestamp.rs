// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Converting between a date and time and a Unix timestamp with `chrono`.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;

/// Converts a `NaiveDateTime` to a timestamp and back.
///
/// # Errors
///
/// Returns an error if a demonstration date, time, or timestamp is out of
/// range.
fn main() -> Result<(), &'static str> {
    // Create a `NaiveDateTime` object representing November 12, 2017, at
    // 17:33:44.
    let date_time = NaiveDate::from_ymd_opt(2017, 11, 12)
        .ok_or("invalid demonstration date")?
        .and_hms_opt(17, 33, 44)
        .ok_or("invalid demonstration time")?;
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {date_time} is
    {}.",
        date_time.and_utc().timestamp()
    );

    // Create a `DateTime` object representing the time one billion seconds
    // after the Unix epoch.
    let date_time_after_a_billion_seconds =
        DateTime::<Utc>::from_timestamp(1_000_000_000, 0)
            .ok_or("timestamp is outside the supported range")?;
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {date_time_after_a_billion_seconds}."
    );
    Ok(())
}

// ANCHOR_END: example

/// # Errors
///
/// Returns an error if a demonstration date, time, or timestamp is out of
/// range.
pub fn run() -> Result<(), &'static str> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrono_timestamp_conversion_runs() -> Result<(), &'static str> {
        main()
    }

    #[test]
    fn known_timestamps_convert_in_both_directions() {
        let dt = NaiveDate::from_ymd_opt(2017, 11, 12)
            .expect("the documented date is valid")
            .and_hms_opt(17, 33, 44)
            .expect("the documented time is valid");
        assert_eq!(dt.and_utc().timestamp(), 1_510_508_024);
        let billion = DateTime::<Utc>::from_timestamp(1_000_000_000, 0)
            .expect("one billion seconds after the epoch is in range");
        assert_eq!(billion.to_string(), "2001-09-09 01:46:40 UTC");
        assert_eq!(
            DateTime::<Utc>::from_timestamp(0, 0)
                .expect("the Unix epoch is in range")
                .to_rfc3339(),
            "1970-01-01T00:00:00+00:00"
        );
    }

    #[test]
    fn out_of_range_inputs_are_rejected() {
        assert!(NaiveDate::from_ymd_opt(2017, 13, 1).is_none());
        assert!(NaiveDate::from_ymd_opt(2017, 2, 29).is_none());
        assert!(DateTime::<Utc>::from_timestamp(i64::MAX, 0).is_none());
    }
}
