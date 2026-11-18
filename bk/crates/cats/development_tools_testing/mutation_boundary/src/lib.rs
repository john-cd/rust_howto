//! A tiny boundary with a test that initially misses its equality case.

// ANCHOR: example
/// Classify values below ten as small.
#[must_use]
pub fn classify(value: u8) -> &'static str {
    if value < 10 { "small" } else { "large" }
}
// ANCHOR_END: example

/// Checks values below the threshold and the equality boundary at ten.
#[cfg(test)]
mod tests {
    use super::classify;

    /// Checks that classifies a small value.
    #[test]
    fn classifies_a_small_value() {
        assert_eq!(classify(5), "small");
        assert_eq!(classify(10), "large");
    }
}
