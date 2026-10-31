// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates parsing various date and time strings into `chrono` types.
//!
//! Dates arrive as text in many layouts: RFC 2822 (emails), RFC 3339 / ISO 8601
//! (APIs) and custom ones. Parsing requires you to know the format, and an
//! error means the text did not match it.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::format::ParseError;

/// Demonstrates parsing various date and time strings into `chrono` types.
fn main() -> Result<(), ParseError> {
    // Parse a date and time string in RFC 2822 format.
    let rfc2822 =
        DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{rfc2822}");

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{rfc3339}");

    let custom = DateTime::parse_from_str(
        "5.8.1994 8:00 am +0000", // Custom format string
        "%d.%m.%Y %H:%M %P %z",   // Custom format string
    )?;
    println!("{custom}");

    // Parse a time-only string.
    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{time_only}");

    // Parse a date-only string.
    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{date_only}");

    // Parse a date and time string without a timezone.
    let no_timezone = NaiveDateTime::parse_from_str(
        "2015-09-05 23:56:04",
        "%Y-%m-%d %H:%M:%S",
    )?;
    println!("{no_timezone}");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), ParseError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrono_parses_the_documented_date_formats() -> Result<(), ParseError> {
        main()
    }

    #[test]
    fn parsed_values_have_the_expected_components() -> Result<(), ParseError> {
        let rfc2822 =
            DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
        assert_eq!(rfc2822.to_rfc3339(), "2003-07-01T10:52:37+02:00");
        let rfc3339 =
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
        assert_eq!(rfc3339.timestamp(), 851_042_397);
        let custom = DateTime::parse_from_str(
            "5.8.1994 8:00 am +0000",
            "%d.%m.%Y %H:%M %P %z",
        )?;
        assert_eq!(custom.to_rfc3339(), "1994-08-05T08:00:00+00:00");
        assert_eq!(
            NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?.to_string(),
            "23:56:04"
        );
        assert_eq!(
            NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?.to_string(),
            "2015-09-05"
        );
        assert_eq!(
            NaiveDateTime::parse_from_str(
                "2015-09-05 23:56:04",
                "%Y-%m-%d %H:%M:%S"
            )?
            .to_string(),
            "2015-09-05 23:56:04"
        );
        Ok(())
    }

    #[test]
    fn malformed_input_is_a_parse_error() {
        assert!(DateTime::parse_from_rfc3339("not a date").is_err());
        assert!(DateTime::parse_from_rfc2822("1996-12-19").is_err());
        assert!(NaiveDate::parse_from_str("2015-13-45", "%Y-%m-%d").is_err());
        assert!(NaiveTime::parse_from_str("25:00:00", "%H:%M:%S").is_err());
        assert!(NaiveDate::parse_from_str("2015-09-05", "%d/%m/%Y").is_err());
    }
}
