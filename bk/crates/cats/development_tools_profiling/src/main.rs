//! Memory usage and timing examples.
//!
//! TODO Reviewed ??-??-2026.

/// Examples that analyze memory usage and measure time.
mod memory_usage_analysis {
    pub mod dhat;
    pub mod measure_time;
}

/// Runs the `dhat` and the time measurement examples.
fn main() {
    memory_usage_analysis::dhat::run();
    memory_usage_analysis::measure_time::run();
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
