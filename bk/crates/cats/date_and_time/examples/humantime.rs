// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parse and format human-readable durations with `humantime`.
//!
//! ### Overview
//! The `humantime` crate parses string representations of time durations (such
//! as `"2h 30m 15s"`, `"1 day 2 hours"`, or `"150ms"`) into standard
//! `std::time::Duration` instances, and formats `std::time::Duration` or
//! `std::time::SystemTime` into human-friendly strings.

use std::time::Duration;

/// Parses a human-readable duration and formats it again.
///
/// # Errors
///
/// Returns an error if the sample duration is invalid.
fn main() -> anyhow::Result<()> {
    let human_text = "2h 30m 15s";
    let duration: Duration = humantime::parse_duration(human_text)?;
    let formatted = humantime::format_duration(duration).to_string();
    println!("{human_text} is {formatted}");

    Ok(())
}
// ANCHOR_END: example

/// Runs the humantime example.
#[allow(
    clippy::main_recursion,
    reason = "run() is the shared example runner; main() is the book snippet"
)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses a duration and formats it back to human-readable text.
    #[test]
    fn humantime_parses_and_formats_a_duration() -> anyhow::Result<()> {
        let duration = humantime::parse_duration("2h 30m 15s")?;
        assert_eq!(duration, Duration::from_secs(9_015));
        assert_eq!(
            humantime::format_duration(duration).to_string(),
            "2h 30m 15s"
        );

        main()
    }
}
