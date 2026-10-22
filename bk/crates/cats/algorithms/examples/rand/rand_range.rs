#![allow(dead_code)]

// TODO Reviewed ??-??-2026
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
