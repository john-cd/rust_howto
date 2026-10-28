// TODO Reviewed ??-??-2026.
// TODO explain that This function is optimised for the case that only a single
// sample is made from the given range. All supported
// types may be sampled with low..high_exclusive (Range) and low..=high
// (RangeInclusive) syntax. Unsigned integer types also support ..high_exclusive
// (RangeTo) and ..=high (RangeToInclusive) syntax.

// ANCHOR: example
//! Demonstrates generating random numbers within a specified range.

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
