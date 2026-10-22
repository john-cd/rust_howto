//! Generates a random `f64` with the `rand` crate.
//!
//! TODO Reviewed ??-??-2026

// ANCHOR: example
use rand::RngExt;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
fn example() {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    // Generate a random `f64`.
    println!("Random f64: {}", rng.random::<f64>());
}

/// Runs `example`.
fn main() {
    example();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the example runs without panicking.
    #[test]
    fn test() {
        example();
    }
}
