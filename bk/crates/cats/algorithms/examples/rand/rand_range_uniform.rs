// TODO Reviewed ??-??-2026.
// TODO Explain: the Uniform distribution type
// may be faster if sampling from the same range repeatedly.

// ANCHOR: example
//! Generates random numbers in ranges with the `rand` crate.

use rand::distr::Distribution as _;
use rand::distr::Uniform;

/// Prints random numbers drawn from several ranges.
fn main() {
    // Create a random number generator:
    let mut rng = rand::rng();

    // Create a uniform distribution over the range [1, 7):
    let die = Uniform::try_from(1..7).expect("the sample range should be valid");

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

    #[test]
    fn test_rand_range1_example_entry_point() {
        main();
    }
}
