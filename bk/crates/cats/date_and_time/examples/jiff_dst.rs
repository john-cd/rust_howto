//! Resolve a repeated or skipped wall-clock time explicitly.

use jiff::ToSpan;
use jiff::civil::date;
use jiff::tz::TimeZone;

// ANCHOR: example
fn schedule() -> Result<(), jiff::Error> {
    // The bundled IANA database makes this fixture independent of the host's zoneinfo.
    let zone = TimeZone::get("America/New_York")?;
    let repeated = date(2024, 11, 3).at(1, 30, 0, 0);
    let ambiguous = zone.to_ambiguous_zoned(repeated);
    assert!(ambiguous.clone().unambiguous().is_err());
    let first = ambiguous.clone().earlier()?;
    let second = ambiguous.later()?;
    assert_eq!(
        first.to_string(),
        "2024-11-03T01:30:00-04:00[America/New_York]"
    );
    assert_eq!(
        second.to_string(),
        "2024-11-03T01:30:00-05:00[America/New_York]"
    );
    assert_ne!(first.timestamp(), second.timestamp());

    let skipped = date(2024, 3, 10).at(2, 30, 0, 0);
    let gap = zone.to_ambiguous_zoned(skipped);
    assert!(gap.clone().unambiguous().is_err());
    let after_gap = gap.later()?;
    assert_eq!(
        after_gap.to_string(),
        "2024-03-10T03:30:00-04:00[America/New_York]"
    );

    let start = date(2024, 3, 9).at(0, 0, 0, 0).to_zoned(zone)?;
    let calendar_day = start.checked_add(1.day())?;
    let elapsed_day = start.checked_add(24.hours())?;
    assert_eq!(
        calendar_day.to_string(),
        "2024-03-10T00:00:00-05:00[America/New_York]"
    );
    assert_eq!(elapsed_day, calendar_day);
    let following_calendar_day = calendar_day.checked_add(1.day())?;
    let following_elapsed_day = calendar_day.checked_add(24.hours())?;
    assert_eq!(
        following_calendar_day.to_string(),
        "2024-03-11T00:00:00-04:00[America/New_York]"
    );
    assert_eq!(
        following_elapsed_day.to_string(),
        "2024-03-11T01:00:00-04:00[America/New_York]"
    );
    Ok(())
}
// ANCHOR_END: example

fn main() -> Result<(), jiff::Error> {
    schedule()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    /// Checks that disambiguation and calendar arithmetic.
    #[test]
    fn disambiguation_and_calendar_arithmetic() -> Result<(), jiff::Error> {
        super::schedule()
    }
}
