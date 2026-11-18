// TODO Reviewed 11-17-2026.

// ANCHOR: example
//! Convert between common standard library collections using iterators.
//!
//! `collect` builds a destination collection from an iterator. Hash maps and
//! sets do not guarantee iteration order, so sort their values when output
//! order matters.

use std::collections::HashMap;
use std::collections::HashSet;

/// Collects iterators into sets and maps, sorting set output for deterministic checks.
fn main() {
    let numbers = [1, 2, 3, 2, 1];
    let unique: HashSet<i32> = numbers.into_iter().collect();
    let mut unique_values: Vec<_> = unique.into_iter().collect();
    unique_values.sort_unstable();
    assert_eq!(unique_values, [1, 2, 3]);

    let pairs = [("one", 1), ("two", 2), ("three", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    assert_eq!(map.get("two"), Some(&2));

    let words = ["apple", "banana", "cherry"];
    let indexed: HashMap<usize, &str> = words.into_iter().enumerate().collect();
    assert_eq!(indexed.get(&1), Some(&"banana"));

    println!("Converted arrays and iterators into sets and maps.");
}
// ANCHOR_END: example

/// Checks the collection conversions through the example's assertions.
#[cfg(test)]
mod tests {
    /// Runs the conversion example and checks its assertions.
    #[test]
    fn collection_conversions_succeed() {
        super::main();
    }
}
