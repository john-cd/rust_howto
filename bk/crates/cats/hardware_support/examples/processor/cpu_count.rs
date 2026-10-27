#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to get the number of logical cores on the
//! system. It uses the `num_cpus` crate.

/// This example demonstrates how to get the number of logical cores on the
/// system. It uses the `num_cpus` crate.
fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}

// ANCHOR_END: example

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
}
