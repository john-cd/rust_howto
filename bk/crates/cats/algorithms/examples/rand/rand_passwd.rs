#![allow(dead_code)]
// ANCHOR: example
use rand::Rng;
use rand::distr::Alphanumeric;

/// Generate a random string of 30 alphanumeric characters.
fn main() {
    let rand_string: String = rand::rng()
        .sample_iter(&Alphanumeric)
        // Take the first 30 characters.
        .take(30)
        .map(char::from)
        .collect();
    println!("{rand_string}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
