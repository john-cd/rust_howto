//! Runs the duration profiling example.
//!
//! TODO Reviewed ??-??-2026.

mod profile;

/// Runs the duration example.
fn main() {
    profile::run();
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
