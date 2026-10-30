// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Formatting the current time with `chrono`.

use chrono::DateTime;
use chrono::Utc;

/// Prints the current time in several formats.
fn main() {
    // Get the current time in UTC.
    let now: DateTime<Utc> = Utc::now();

    // Print the current time in various formats.
    println!("UTC now is: {now}");
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the format example through its entry point.
    #[test]
    fn test_format_example_entry_point() {
        main();
    }

    #[test]
    fn a_fixed_instant_formats_in_every_documented_style() {
        use chrono::TimeZone as _;

        let t: DateTime<Utc> = Utc
            .with_ymd_and_hms(2017, 11, 12, 17, 33, 44)
            .single()
            .expect("the documented UTC instant is valid");
        assert_eq!(t.to_string(), "2017-11-12 17:33:44 UTC");
        assert_eq!(t.to_rfc2822(), "Sun, 12 Nov 2017 17:33:44 +0000");
        assert_eq!(t.to_rfc3339(), "2017-11-12T17:33:44+00:00");
        assert_eq!(
            t.format("%a %b %e %T %Y").to_string(),
            "Sun Nov 12 17:33:44 2017"
        );
    }

    #[test]
    fn rfc3339_output_parses_back_to_the_same_instant() {
        let now = Utc::now();
        let parsed = DateTime::parse_from_rfc3339(&now.to_rfc3339())
            .expect("Chrono's RFC 3339 output parses successfully");
        assert_eq!(parsed.timestamp(), now.timestamp());
    }
}
