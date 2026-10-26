#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! BEWARE: `fastrand` is NOT cryptographically secure.

use std::iter::repeat_with;

use fastrand::Rng;

/// Generates random booleans, numbers and strings with `fastrand`, then samples
/// and shuffles values.
fn main() {
    // Seed this independent generator to get reproducible results on every run:
    let mut rng = Rng::with_seed(42);
    // Generate a boolean, number, digit or character from this generator:
    println!("Boolean: {} ", rng.bool());
    // Generate a random u32:
    println!("u32: {} ", rng.u32(..100));
    // Generate a random `Vec` or `String`:
    let mut v: Vec<i32> = repeat_with(|| rng.i32(..)).take(5).collect();
    println!("Vector: {v:?}");
    let s: String = repeat_with(|| rng.alphanumeric()).take(10).collect();
    println!("String: {s}");
    // Sample values from an array or range:
    println!(
        "Two samples from the 0..20 interval: {:?}",
        rng.choose_multiple(0..20, 2)
    );
    // Choose a random element in a vector or array:
    let i = rng.usize(..v.len());
    let elem = v[i];
    println!("Random element: {elem}");
    // Shuffle an array:
    rng.shuffle(&mut v);
    println!("Shuffled vector: {v:?}");
}

// ANCHOR_END: example

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
