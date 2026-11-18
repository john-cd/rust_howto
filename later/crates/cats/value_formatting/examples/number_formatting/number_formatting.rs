// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Displays cents as a signed decimal amount.

/// Displays a signed amount with two decimal places.
fn main() {
    // Keep settled money in minor units; format only at the display boundary.
    let cents: i64 = -123_456;
    let sign = if cents < 0 { "-" } else { "" };
    let magnitude = cents.unsigned_abs();
    let display = format!("{sign}{}.{:02}", magnitude / 100, magnitude % 100);
    assert_eq!(display, "-1234.56");
}
// ANCHOR_END: example

/// Runs the numeric formatting example.
pub fn run() {
    main();
}

/// Checks that negative cents display with the correct sign and scale.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that negative cents display with the correct sign and scale.
    #[test]
    fn formats_values_without_using_display_text_for_arithmetic() {
        main();
    }
}
