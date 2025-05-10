// ANCHOR: example
//! Use the fastrand crate.
//!
//! `fastrand` is a simple and fast random number generator.
//! BEWARE: It is NOT cryptographically secure.

use std::iter::repeat_with;

use fastrand::Rng;

fn main() {
    // To get reproducible results on every run,
    // pick an arbitrary number as seed.
    fastrand::seed(42);
    // Call a function to generate a number, a boolean, a digit, a character...
    println!("Boolean: {} ", fastrand::bool());
    // Or create a new random number generator:
    let mut rng = Rng::new();
    // Generate a random i32.
    println!("u32: {} ", rng.u32(..100));
    // Generate a random `Vec` or `String`:
    let mut v: Vec<i32> = repeat_with(|| rng.i32(..)).take(5).collect();
    println!("Vector: {:?}", v);
    let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
    println!("String: {}", s);
    // Sample values from an array:
    println!(
        "Two samples from the 0..20 interval: {:?}",
        fastrand::choose_multiple(0..20, 2)
    );
    // Choose a random element in a vector or array:
    let i = fastrand::usize(..v.len());
    let elem = v[i];
    println!("Random element: {}", elem);
    // Shuffle an array:
    fastrand::shuffle(&mut v);
    println!("Shuffled vector: {:?}", v);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
