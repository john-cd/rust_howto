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
    fastrand::seed(7);
    // Call a function to generate a number, a boolean, a digit, a character...
    println!("{} ", fastrand::bool());
    // Or create a new random number generator:
    let mut rng = Rng::new();
    for _ in 0..3 {
        // Generate a random i32.
        print!("{} ", rng.u32(..100));
    }
    println!();
    // Generate a random Vec or String:
    let mut v: Vec<i32> = repeat_with(|| rng.i32(..)).take(10).collect();
    println!("{:?}", v);
    let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
    println!("{}", s);
    // Sample values from an array:
    println!("{:?}", fastrand::choose_multiple(0..20, 12));
    // Choose a random element in an array:
    let i = fastrand::usize(..v.len());
    let elem = v[i];
    println!("{}", elem);
    // Shuffle an array:
    fastrand::shuffle(&mut v);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
