// TODO Reviewed ??-??-2026.

// ANCHOR: timestamp
/// Shows that a zone conversion changes the displayed time and preserves the instant.
///
/// # Errors
///
/// Returns an error if the timestamp cannot be parsed or the time zone cannot be loaded.
fn show_timestamp() -> Result<(), jiff::Error> {
    let instant: jiff::Timestamp = "2025-01-15T12:00:00Z".parse()?;
    let new_york = instant.in_tz("America/New_York")?;

    assert_eq!(
        new_york.to_string(),
        "2025-01-15T07:00:00-05:00[America/New_York]"
    );
    assert_eq!(new_york.timestamp(), instant);
    Ok(())
}
// ANCHOR_END: timestamp

// ANCHOR: duration
/// Parses and formats a fixed elapsed duration.
///
/// # Errors
///
/// Returns an error if the duration string cannot be parsed.
fn show_duration() -> Result<(), jiff::Error> {
    let elapsed: jiff::SignedDuration = "2h 30m 15s".parse()?;
    assert_eq!(elapsed.as_secs(), 9_015);
    assert_eq!(
        format!("{elapsed:#}").parse::<jiff::SignedDuration>()?,
        elapsed
    );
    Ok(())
}
// ANCHOR_END: duration

/// Runs both Jiff examples.
///
/// # Errors
///
/// Returns the first parsing or time-zone error.
fn main() -> Result<(), jiff::Error> {
    show_timestamp()?;
    show_duration()
}

#[cfg(test)]
mod tests {
    /// Verifies the fixed examples without reading the clock or the local time zone.
    #[test]
    fn jiff_examples_match_expected_values() -> Result<(), jiff::Error> {
        super::main()
    }
}
