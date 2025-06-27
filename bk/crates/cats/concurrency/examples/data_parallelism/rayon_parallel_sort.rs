#![allow(dead_code)]
// ANCHOR: example
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::rng;
use rayon::prelude::*;

/// This example demonstrates parallel sorting of a vector of strings using
/// Rayon.
fn main() {
    let mut vec = vec![String::new(); 100];

    // Generate random strings in parallel and populate the vector.
    vec.par_iter_mut().for_each(|p| {
        // Create a thread-local random number generator.
        let mut rng = rng();
        *p = (0..5).map(|_| rng.sample(Alphanumeric) as char).collect();
    });
    vec.par_sort_unstable();
    println!("{vec:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
