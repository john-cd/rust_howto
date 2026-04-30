// ANCHOR: example
//! This example demonstrates the `humantime` crate for parsing and formatting
//! human-readable durations.

use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Parse a duration from a human-readable string.
    let human_text = "2h 30m 15s";
    let duration: Duration = humantime::parse_duration(human_text)?;

    println!("Parsed duration from '{human_text}': {duration:?}");

    // Format the parsed duration back into a human-readable string.
    let formatted = humantime::format_duration(duration).to_string();
    println!("Formatted duration: {formatted}");

    // Compute and format half of the duration.
    let half_duration = duration.checked_div(2).unwrap_or_default();
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

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        main()
    }
}
// TODO add to a chapter on date and time
