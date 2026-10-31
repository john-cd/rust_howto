//! Runs the discrete mathematics example.
//!
//! TODO Reviewed ??-??-2026.

mod set_operations;

/// Runs the set operations example.
fn main() {
    set_operations::run();
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
