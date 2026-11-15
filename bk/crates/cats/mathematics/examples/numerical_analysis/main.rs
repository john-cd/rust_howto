//! Runs the numerical analysis example.
//!
//! TODO Reviewed ??-??-2026.

/// Find a root of a function numerically.
mod root_finding;
/// Runs the example.
/// Runs the root-finding example.

fn main() {
    root_finding::run();
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
