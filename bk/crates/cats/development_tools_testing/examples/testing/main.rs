//! Runs the `insta` snapshot testing example.
//!
//! TODO Reviewed ??-??-2026.

mod insta;

/// Runs the testing example.
fn main() {
    insta::run();
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
