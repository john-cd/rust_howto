//! Runs the test runner example.
//!
//! TODO Reviewed ??-??-2026.

/// Writing tests that pass, fail, return a `Result` or are ignored.
mod tests1;
/// Runs the example.
/// Runs the test runners example.
fn main() {
    tests1::run();
}

/// panic fails the test.
#[cfg(test)]
mod tests {
    use super::*;

    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
