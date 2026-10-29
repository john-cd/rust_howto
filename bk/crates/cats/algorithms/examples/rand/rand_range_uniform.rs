// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generates random numbers in ranges with the `rand` crate.
//!
//! ### Performance Advantage of `Uniform` Distribution
//! When sampling multiple random values from the same range repeatedly (such as
//! in loops or simulations), pre-computing a `rand::distr::Uniform`
//! distribution avoids repeating initialization and pre-computation costs. This
//! makes `Uniform` significantly faster than repeatedly invoking `random_range`
//! in hot loops.

use rand::distr::Distribution as _;
use rand::distr::Uniform;

/// Prints random numbers drawn from several ranges.
fn main() {
    // Create a random number generator:
    let mut rng = rand::rng();

    // Create a uniform distribution over the range [1, 7):
    let die =
        Uniform::try_from(1..7).expect("the sample range should be valid");

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
    fn test_rand_range_uniform_example_entry_point() {
        main();
    }

    #[test]
    fn uniform_die_rolls_stay_in_range_and_cover_all_faces() {
        let mut rng = rand::rng();
        let die = Uniform::try_from(1..7).expect("valid range");
        let rolls: Vec<i32> = (0..2000).map(|_| die.sample(&mut rng)).collect();
        assert!(rolls.iter().all(|r| (1..=6).contains(r)));
        for face in 1..=6 {
            assert!(rolls.contains(&face), "face {face} never rolled");
        }
    }

    #[test]
    fn empty_range_is_rejected() {
        assert!(Uniform::try_from(5..5).is_err());
    }
}
