//! Runs the `FromStr` parsing example.
//!
//! TODO Reviewed ??-??-2026.

/// Parsing a color from a string by implementing `FromStr`.
mod from_str;
/// Runs the example.
/// Runs the string parsing example.
fn main() {
    from_str::run();
}

/// Verifies that the `FromStr` parsing example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
