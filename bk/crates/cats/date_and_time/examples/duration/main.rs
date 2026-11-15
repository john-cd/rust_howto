//! Runs the duration profiling example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates how to profile the execution time of a function
mod profile;
/// Runs the example.
/// Runs the duration example.

fn main() {
    profile::run();
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
