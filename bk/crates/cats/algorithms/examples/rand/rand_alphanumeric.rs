#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generate a random string of 20 alphanumeric characters.

use rand::RngExt as _;
use rand::distr::Alphanumeric;
use rand::distr::SampleString as _;

/// Generate a random string of 20 alphanumeric characters.
fn main() {
    let rand_string: String = rand::rng()
        // Create an iterator that generates values using the `Alphanumeric`
        // distribution:
        .sample_iter(&Alphanumeric)
        // Take the first 20 characters:
        .take(20)
        .map(char::from)
        .collect();
    println!("Random string: {rand_string}");

    // Or use the `SampleString` trait:
    let rand_string2 = Alphanumeric.sample_string(&mut rand::rng(), 20);
    println!("Another random string: {rand_string2}");
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
