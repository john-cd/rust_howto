// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Displays signed amounts stored in cents.

/// Formats cents as a decimal amount with two fractional digits.
fn format_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let magnitude = cents.unsigned_abs();
    format!("{sign}{}.{:02}", magnitude / 100, magnitude % 100)
}

/// Displays an amount stored in cents.
pub fn run() {
    println!("{}", format_cents(-123_456));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::format_cents;

    /// Preserves the sign, fixed scale, and full range of signed cents.
    #[test]
    fn formats_signed_cents() {
        assert_eq!(format_cents(0), "0.00");
        assert_eq!(format_cents(105), "1.05");
        assert_eq!(format_cents(-5), "-0.05");
        assert_eq!(format_cents(-123_456), "-1234.56");
        assert_eq!(format_cents(i64::MIN), "-92233720368547758.08");
    }
}
