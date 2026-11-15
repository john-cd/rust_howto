//! Memory usage and timing examples.
//!
//! TODO Reviewed ??-??-2026.

/// Examples that analyze memory usage and measure time.
mod memory_usage_analysis {

    /// This module provides the `dhat` implementation used by its parent.
    pub mod dhat;

    /// This module provides the `measure_time` implementation used by its parent.
    pub mod measure_time;
}

/// Runs both the heap profiling and execution-time demonstrations.
fn main() {
    memory_usage_analysis::dhat::run();
    memory_usage_analysis::measure_time::run();
}

/// Calls both profiling demonstrations through the entry point.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both demonstrations return control to the entry point.
    #[test]
    fn test_main_runs() {
        main();
    }
}
