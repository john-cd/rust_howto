#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Hash a value with `foldhash` using a fixed seed for reproducibility.

use std::hash::BuildHasher as _;

/// Computes a reproducible, non-cryptographic hash for a string.
fn main() {
    let state = foldhash::fast::FixedState::with_seed(42);
    let first_hash = state.hash_one("example key");
    let same_hash = state.hash_one("example key");

    assert_eq!(first_hash, same_hash);
    println!("Hash: {first_hash}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example's fixed seed produces repeatable output.
    #[test]
    fn fixed_seed_repeats_the_same_hash() {
        main();
    }
}
