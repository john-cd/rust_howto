// ANCHOR: example
//! Demonstrates how to implement a custom distribution for the `rand` crate.

use rand::Rng;
use rand::distr::Distribution;
use rand::distr::StandardUniform;

/// A simple struct representing a point in 2D space.
#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

// Required by:
// pub fn random<T>() -> T
// This implementation allows generating random `Point` instances.
impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.random::<(i32, i32)>();
        Point { x, y }
    }
}

fn main() {
    // Access a fast, pre-initialized generator.
    let mut rng = rand::rng();
    // Generate a random tuple of (i32, bool, f64).
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    println!("Random tuple: {:?}", rand_tuple);

    // Generate a random Point using the custom distribution.
    let rand_point: Point = rng.random();
    println!("Random Point: {:?}", rand_point);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
