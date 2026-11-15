//! Runs the case-conversion example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates identifier case conversion with heck.
mod case_conversion;
/// Runs the example.
/// Runs the case-conversion example.
fn main() {
    case_conversion::run();
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
