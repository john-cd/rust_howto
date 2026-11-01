//! Runs the numerical analysis example.
//!
//! TODO Reviewed ??-??-2026.

mod root_finding;

/// Runs the root-finding example.
fn main() {
    root_finding::run();
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
