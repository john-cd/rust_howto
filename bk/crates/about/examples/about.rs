// Reviewed 10-27-2026.

// ANCHOR: example
//! Generates a random `f64` with the `rand` crate.

use rand::RngExt as _;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
fn main() {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    println!("Random f64: {}", rng.random::<f64>());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_about_example() {
        main();
    }
}
