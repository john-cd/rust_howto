// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generates a random `f64` with the `rand` crate.

use rand::RngExt as _;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
fn example() {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    // Generate a random `f64`.
    println!("Random f64: {}", rng.random::<f64>());
}

/// Runs the about example.
fn main() {
    example();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the about example entry point.
    #[test]
    fn test_about_example() {
        example();
    }
}
