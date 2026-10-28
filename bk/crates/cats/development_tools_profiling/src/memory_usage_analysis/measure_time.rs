// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `measure_time` is a simple crate for measuring the execution time of code
//! blocks using the `measure_time!` macro.

use std::thread;
use std::time::Duration;

use measure_time::print_time;

/// Measure the execution time of a code block.
fn measure_execution_time() {
    // The `print_time!` macro prints the elapsed time when it goes out of
    // scope.
    print_time!("execution_time");

    // Simulate some work:
    thread::sleep(Duration::from_millis(100));
}

/// Runs the measure time example.
fn main() {
    measure_execution_time();
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the measure time example through its entry point.
    #[test]
    fn test_measure_time_example_entry_point() {
        main();
    }
}
