// Reviewed 10-27-2026.
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generates a random `f64` with the `rand` crate.
//!
//! Random numbers come from a generator. `rand` seeds it from the operating
//! system, so each run differs; use a fixed seed in tests to make results
//! reproducible.

use rand::RngExt as _;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
/// Returns a random `f64` in `[0, 1)`.
fn random_f64() -> f64 {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    rng.random::<f64>()
}

/// Generates a random `f64` with the `rand` crate.
fn main() {
    run();
}

/// Generates and prints a random floating-point value.
fn example() {
    println!("Random f64: {}", random_f64());
}
// ANCHOR_END: example

/// Generates a random `f64` with the `rand` crate.
fn run() {
    example();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
    /// Verifies random f64 is in unit interval.
    #[test]
    fn test_random_f64_is_in_unit_interval() {
        for _ in 0..1000 {
            let x = random_f64();
            assert!((0.0..1.0).contains(&x), "{x} is out of range");
        }
    }
}
