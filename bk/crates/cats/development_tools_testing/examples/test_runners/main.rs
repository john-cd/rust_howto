//! Runs the test runner example.
//!
//! TODO Reviewed ??-??-2026.

mod tests1;

/// Runs the test runners example.
fn main() {
    tests1::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example; each module's `run()` asserts its own results, so a
    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
