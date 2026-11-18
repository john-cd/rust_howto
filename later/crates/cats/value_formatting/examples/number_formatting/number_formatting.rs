// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Formats numeric values for display without changing their stored values.

/// Demonstrates standard formatting syntax and exact minor-unit display.
fn main() {
    let measurement = 12_345.6789;
    assert_eq!(format!("{measurement:.2}"), "12345.68");
    assert_eq!(format!("{measurement:>12.2}"), "    12345.68");

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

/// Checks decimal precision, alignment, and negative minor-unit formatting.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks decimal precision, alignment, and negative minor-unit formatting.
    #[test]
    fn formats_values_without_using_display_text_for_arithmetic() {
        main();
    }
}
