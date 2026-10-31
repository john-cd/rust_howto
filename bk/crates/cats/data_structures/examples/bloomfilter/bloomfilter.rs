// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A Bloom filter, a compact set that can give false positives but no false
//! negatives.
//!
//! A Bloom filter answers "have I seen this item?" using very little memory. It
//! may say yes for an item it has not seen (a false positive), but never says
//! no for one it has. It is used to skip expensive lookups, for example to
//! avoid disk reads for keys that do not exist.

use bloomfilter::Bloom;

/// Inserts items into a Bloom filter and checks for membership.
fn main() {
    let items_count = 100;
    let mut bloom = Bloom::new_for_fp_rate(items_count, 0.01)
        .expect("the bloomfilter example invariant should hold");

    bloom.set(&"item1");
    bloom.set(&"item2");

    assert!(bloom.check(&"item1"));
    assert!(bloom.check(&"item2"));
    assert!(!bloom.check(&"item3"));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the bloomfilter example through its entry point.
    #[test]
    fn test_bloomfilter_example_entry_point() {
        main();
    }
}
