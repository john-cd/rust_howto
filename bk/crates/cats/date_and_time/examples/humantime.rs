// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the `humantime` crate for parsing and formatting
//! human-readable durations.

use std::time::Duration;

/// Parses and formats durations and timestamps in a human-readable form.
///
/// # Errors
///
/// Returns an error if either sample duration or timestamp is invalid.
fn main() -> anyhow::Result<()> {
    // Parse a duration from a human-readable string.
    let human_text = "2h 30m 15s";
    let duration: Duration = humantime::parse_duration(human_text)?;

    println!("Parsed duration from '{human_text}': {duration:?}");

    // Format the parsed duration back into a human-readable string.
    let formatted = humantime::format_duration(duration).to_string();
    println!("Formatted duration: {formatted}");

    // Compute and format half of the duration.
    let half_duration = duration / 2;
    println!(
        "Half the duration is: {}",
        humantime::format_duration(half_duration)
    );

    // Format a short duration example.
    let fast_duration = Duration::from_millis(750);
    println!(
        "Fast operation time: {}",
        humantime::format_duration(fast_duration)
    );

    // Parse an RFC 3339 timestamp and format it back to the same
    // representation.
    let timestamp_text = "2018-01-01T12:53:00Z";
    let timestamp = humantime::parse_rfc3339(timestamp_text)?;
    let formatted_timestamp = humantime::format_rfc3339(timestamp).to_string();
    assert_eq!(formatted_timestamp, timestamp_text);
    println!(
        "Formatted timestamp '{timestamp_text}' as '{formatted_timestamp}'."
    );

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humantime_parses_and_formats_durations_and_timestamps()
    -> anyhow::Result<()> {
        main()
    }
}
// TODO add to a chapter on date and time
