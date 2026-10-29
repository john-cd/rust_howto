// Reviewed 10-27-2026.

// ANCHOR: example
//! Generates a random `f64` with the `rand` crate.

use rand::RngExt as _;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
/// Returns a random `f64` in `[0, 1)`.
fn random_f64() -> f64 {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    rng.random::<f64>()
}

fn main() {
    println!("Random f64: {}", random_f64());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_random_f64_is_in_unit_interval() {
        for _ in 0..1000 {
            let x = random_f64();
            assert!((0.0..1.0).contains(&x), "{x} is out of range");
        }
    }
}
