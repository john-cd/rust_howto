#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Fast, reproducible non-cryptographic hashing using `foldhash` with a fixed
//! seed.
//!
//! ### What is FoldHash?
//! `foldhash` is an extremely fast, non-cryptographic hash function optimized
//! for in-memory lookup tables such as `HashMap` and `HashSet`.
//!
//! ### How Does It Work?
//! FoldHash uses hardware-accelerated 64-bit integer multiplication and folding
//! arithmetic operations to process byte chunks quickly while maintaining high
//! bit dispersion and low collision rates.
//!
//! ### Fixed Seed vs. Random Seed
//! - **Default HashMap (SipHash-1/3)**: Uses randomized per-process seeds to
//!   defend against HashDoS (Hash Denial of Service) collision attacks.
//! - **Fixed Seed (`FixedState`)**: Using a explicit, fixed 64-bit seed
//!   guarantees that identical inputs generate the exact same 64-bit hash code
//!   across multiple runs or program restarts.
//!
//! ### Purpose & Use Cases
//! - Fast hash table keys when resistance to external HashDoS attacks is not
//!   needed.
//! - Reproducible test fixtures, deterministic caching, and compiler symbol
//!   tables.

use std::hash::BuildHasher as _;

/// Demonstrates computing deterministic, reproducible non-cryptographic hash
/// values using `foldhash`.
fn main() {
    // Construct a hasher state configured with an explicit seed value (42).
    let state = foldhash::fast::FixedState::with_seed(42);

    // Hash string slices deterministically.
    let first_hash = state.hash_one("example key");
    let same_hash = state.hash_one("example key");

    // Identical keys with the same seed yield identical hash values.
    assert_eq!(first_hash, same_hash);
    println!("Hash: {first_hash}");
}

// ANCHOR_END: example

/// Runs the `foldhash` example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
/// Unit tests for the `foldhash` algorithm example.
mod tests {
    use super::*;

    /// Verifies that the fixed seed in `foldhash` produces repeatable hash
    /// outputs.
    #[test]
    fn fixed_seed_repeats_the_same_hash() {
        main();
    }
}
