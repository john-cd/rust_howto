#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The same word count with three hash maps: the standard library's, `ahash`'s
//! and `rustc-hash`'s.
//!
//! Trade-offs:
//! - `std::collections::HashMap` uses SipHash, which resists HashDoS
//!   (attacker-chosen keys that all collide). Keep it for keys that come from
//!   untrusted input.
//! - `ahash::AHashMap` is much faster on strings and other larger keys, and
//!   is randomly seeded by default.
//! - `rustc_hash::FxHashMap` is the fastest for small integer or enum keys, is
//!   deterministic (not seeded), and has weak distribution for some key
//!   patterns.
//!
//! `hashbrown` (the table behind `std`'s `HashMap`) and `foldhash` are other
//! options. Measure with `criterion` before switching.

use std::collections::HashMap;

use ahash::AHashMap;
use rustc_hash::FxHashMap;

/// Counts the words with the standard library `HashMap`.
fn count_std(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Counts the words with `AHashMap`, which has the same API as `HashMap`.
fn count_ahash(text: &str) -> AHashMap<&str, usize> {
    let mut counts = AHashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Counts the words with `FxHashMap`, a `HashMap` using the Fx hasher.
fn count_fx(text: &str) -> FxHashMap<&str, usize> {
    let mut counts = FxHashMap::default();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Checks that the three maps agree.
fn main() {
    let text = "the quick brown fox jumps over the lazy dog the end";

    let std_counts = count_std(text);
    let ahash_counts = count_ahash(text);
    let fx_counts = count_fx(text);

    assert_eq!(std_counts["the"], 3);
    assert_eq!(ahash_counts["the"], 3);
    assert_eq!(fx_counts["the"], 3);

    assert_eq!(std_counts.len(), ahash_counts.len());
    assert_eq!(std_counts.len(), fx_counts.len());
    for (word, count) in &std_counts {
        assert_eq!(ahash_counts.get(word), Some(count));
        assert_eq!(fx_counts.get(word), Some(count));
    }

    // Pre-size the map when the number of entries is known.
    let mut by_id: FxHashMap<u32, &str> =
        FxHashMap::with_capacity_and_hasher(16, Default::default());
    by_id.insert(1, "one");
    assert_eq!(by_id.get(&1), Some(&"one"));
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
