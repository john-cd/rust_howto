//! Runs the `criterion` benchmarking example.
//!
//! TODO Reviewed ??-??-2026.

mod criterion;

/// Runs the benchmarking example.
fn main() {
    criterion::run();
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
