// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Estimates π from seeded random points in a square.

use rand::RngExt as _;
use rand::SeedableRng as _;
use rand::rngs::StdRng;

/// Returns a Monte Carlo estimate of π, or `None` when there are no samples.
fn estimate_pi(samples: usize, seed: u64) -> Option<f64> {
    if samples == 0 {
        return None;
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let inside = (0..samples)
        .filter(|_| {
            let x: f64 = rng.random_range(-1.0..1.0);
            let y: f64 = rng.random_range(-1.0..1.0);
            x * x + y * y <= 1.0
        })
        .count();

    Some(4.0 * inside as f64 / samples as f64)
}

/// Prints a seeded estimate of π and checks a broad sampling-error bound.
fn main() {
    let estimate = estimate_pi(100_000, 7).expect("the sample count is nonzero");
    assert!((estimate - std::f64::consts::PI).abs() < 0.02);
    println!("Estimated π: {estimate:.4}");
}
// ANCHOR_END: example

/// Runs the seeded π estimate.
pub fn run() {
    main();
}

/// Checks the sample-count validation and deterministic estimate.
#[cfg(test)]
mod tests {
    use super::*;

    /// Rejects an estimate with no samples.
    #[test]
    fn rejects_zero_samples() {
        assert_eq!(estimate_pi(0, 7), None);
    }

    /// Produces the same estimate for repeated calls with the same seed.
    #[test]
    fn fixed_seed_repeats_the_estimate() {
        assert_eq!(estimate_pi(10_000, 7), estimate_pi(10_000, 7));
    }

    /// Keeps the example estimate near π for its configured sample count.
    #[test]
    fn estimate_is_close_to_pi() {
        main();
    }
}
