// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `divan` is a Rust benchmarking framework.
//!
//!
//! To use `divan`, add the following to your project's `Cargo.toml`:
//! ```toml
//! [dev-dependencies]
//!
//! divan = "0.1.21"
//! [[bench]]
//! name = "divan"
//! harness = false
//! ```
//!
//!
//! Run the benchmark with `cargo bench`.
//! This example demonstrates how to benchmark a recursive Fibonacci function.

/// Define a `fibonacci` function to benchmark.
fn compute_fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => compute_fibonacci(n - 2) + compute_fibonacci(n - 1),
    }
}

/// The benchmarks.
mod bench {
    use super::*;

    /// Register the function above for benchmarking.
    #[divan::bench]
    fn fibonacci() -> u64 {
        compute_fibonacci(divan::black_box(20))
    }
}

/// When `harness` is set to false, we are responsible for defining a `main()`
/// function to run tests and benchmarks.
fn main() {
    run_benchmarks();
}

/// Runs the registered benchmarks when this file is built as a benchmark.
fn run_benchmarks() {
    // Run registered benchmarks.
    #[cfg(not(test))]
    divan::main();
}
// Example adapted from https://nikolaivazquez.com/blog/divan/
// ANCHOR_END: example

/// Runs the example through its program entry point.
pub fn run() {
    run_benchmarks();
}

/// Checks that the benchmark entry point is available in test mode.
#[cfg(test)]
mod tests {

    /// The benchmark entry point compiles when the harness is built for tests.
    #[test]
    fn test_divan_example_entry_point() {
        main();
    }
}
