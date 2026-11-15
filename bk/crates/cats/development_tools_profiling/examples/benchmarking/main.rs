//! Runs the `criterion` benchmarking example.
//!
//! TODO Reviewed ??-??-2026.

/// Run the benchmarks with `cargo bench`: Benchmarks measure how fast code runs.
mod criterion;
/// Runs the example.
/// Runs the benchmarking example.

fn main() {
    criterion::run();
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
