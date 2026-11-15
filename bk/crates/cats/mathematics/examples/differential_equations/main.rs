//! Runs a differential equations example.
//!
//! TODO Reviewed ??-??-2026.

/// Solve an exponential decay equation numerically and compare with the exact
mod exponential_decay;
/// Runs the example.
/// Runs the exponential decay example.

fn main() {
    exponential_decay::run();
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
