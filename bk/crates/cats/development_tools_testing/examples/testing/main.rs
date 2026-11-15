//! Runs the `insta` snapshot testing example.
//!
//! TODO Reviewed ??-??-2026.

/// Snapshot testing with `insta`.
mod insta;
/// Runs the example.
/// Runs the testing example.

fn main() {
    insta::run();
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
