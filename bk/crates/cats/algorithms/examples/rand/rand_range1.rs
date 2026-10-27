#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generates random numbers in ranges with the `rand` crate.

// Import necessary traits and structs from the `rand` crate.
use rand::distr::Distribution as _;
use rand::distr::Uniform;

/// Prints random numbers drawn from several ranges.
fn main() {
    // Create a random number generator:
    let mut rng = rand::rng();
    // Create a uniform distribution over the range [1, 7):
    let die = Uniform::try_from(1..7).expect("sample range should be valid");

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {throw}");
        if throw == 6 {
            break;
        }
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rand range1 example through its entry point.
    #[test]
    fn test_rand_range1_example_entry_point() {
        main();
    }
}
