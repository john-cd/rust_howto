// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parse and format a human-readable duration with `humantime`.

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
// TODO add to a chapter on date and time
