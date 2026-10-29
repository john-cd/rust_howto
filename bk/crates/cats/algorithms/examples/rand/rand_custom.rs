// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to implement a custom distribution for the `rand` crate.

use rand::Rng;
use rand::RngExt as _;
use rand::distr::Distribution;
use rand::distr::StandardUniform;

/// A simple struct representing a point in 2D space.
#[derive(Debug)]
struct Point {
    /// The horizontal coordinate.
    x: i32,
    /// The vertical coordinate.
    y: i32,
}

// This implementation allows generating random `Point` instances.
impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.random::<(i32, i32)>();
        Point { x, y }
    }
}

/// Generates and prints random values, including a custom `Point`.
fn main() {
    // Access the standard random value generator:
    let mut rng = rand::rng();

    // Generate a random tuple of (i32, bool, f64):
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    println!("Random tuple: {rand_tuple:?}");

    // Generate a random `Point` using the custom implementation:
    let rand_point: Point = rng.random();
    println!("Random Point: {rand_point:?}");
    println!("x = {}, y = {}", rand_point.x, rand_point.y);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng as _;
    use rand::rngs::StdRng;

    use super::*;

    #[test]
    fn test_rand_custom_example_entry_point() {
        main();
    }

    #[test]
    fn custom_distribution_is_reproducible_with_a_seeded_rng() {
        let mut a = StdRng::seed_from_u64(1);
        let mut b = StdRng::seed_from_u64(1);
        let p: Point = a.random();
        let q: Point = b.random();
        assert_eq!((p.x, p.y), (q.x, q.y));
    }

    #[test]
    fn custom_distribution_samples_vary() {
        let mut rng = StdRng::seed_from_u64(2);
        let points: Vec<(i32, i32)> = (0..10)
            .map(|_| {
                let p: Point = rng.random();
                (p.x, p.y)
            })
            .collect();
        assert!(points.iter().any(|p| *p != points[0]));
    }
}
