//! Runs a differential equations example.
//!
//! TODO Reviewed ??-??-2026.

mod exponential_decay;

/// Runs the exponential decay example.
fn main() {
    exponential_decay::run();
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
