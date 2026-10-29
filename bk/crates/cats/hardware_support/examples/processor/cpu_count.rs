// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to get the number of logical cores on the
//! system. It uses the `num_cpus` crate.

/// Returns the number of logical cores on the system.
fn logical_cores() -> usize {
    num_cpus::get()
}

/// Prints the number of logical cores on the system.
fn main() {
    println!("Number of logical cores is {}", logical_cores());
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the cpu count example through its entry point.
    #[test]
    fn test_cpu_count_example_entry_point() {
        main();
    }

    /// There is at least one core, and never fewer than the physical cores.
    #[test]
    fn reports_at_least_one_core() {
        assert!(logical_cores() >= 1);
        assert!(logical_cores() >= num_cpus::get_physical());
    }
}
