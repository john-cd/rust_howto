//! Runs the processor example.
//!
//! TODO Reviewed ??-??-2026.

mod cpu_count;

/// Runs the processor example.
fn main() {
    cpu_count::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_runs() {
        main();
        cpu_count::run();
        assert!(num_cpus::get() >= 1);
    }
}
