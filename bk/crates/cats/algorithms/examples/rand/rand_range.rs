// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates generating random numbers within a specified range.
//!
//! ### `Rng::random_range` Optimization and Range Syntax
//! The `random_range` method is optimized for single one-off samples drawn from
//! a range. All supported numeric types can be sampled using standard Range
//! (`low..high`) and RangeInclusive (`low..=high`) expressions. Unsigned
//! integer types also support RangeTo (`..high`) and RangeToInclusive
//! (`..=high`) expressions.

use rand::RngExt as _;

/// Demonstrates generating random numbers within a specified range.
fn main() {
    // Obtain a random number generator:
    let mut rng = rand::rng();

    // Generate a random integer between 0 (inclusive) and 10 (exclusive):
    println!("Integer: {}", rng.random_range(0..10));
    // Generate a random float between 0.0 (inclusive) and 10.0 (exclusive):
    println!("Float: {}", rng.random_range(0.0..10.0));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rand range example through its entry point.
    #[test]
    fn test_rand_range_example_entry_point() {
        main();
    }
}
