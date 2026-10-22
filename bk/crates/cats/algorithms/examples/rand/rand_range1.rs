//! Generates random numbers in ranges with the `rand` crate.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
// Import necessary traits and structs from the `rand` crate.
use rand::distr::Distribution;
use rand::distr::Uniform;

/// Prints random numbers drawn from several ranges.
fn main() {
    // Create a random number generator:
    let mut rng = rand::rng();
    // Create a uniform distribution over the range [1, 7):
    let die = Uniform::try_from(1..7).unwrap();

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {throw}");
        if throw == 6 {
            break;
        }
    }
}

// ANCHOR_END: example

/// Runs the example.
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
