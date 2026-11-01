// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A `LinkedHashMap`, a hash map that remembers insertion order.
//!
//! A linked hash map keeps entries in insertion (or access) order, which makes
//! it the basis of an LRU cache: the oldest entry is at the front, ready to be
//! evicted.

use linked_hash_map::LinkedHashMap;

/// Inserts into a `LinkedHashMap` and iterates in insertion order.
fn main() {
    let mut map = LinkedHashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    assert_eq!(map.len(), 3);
    assert_eq!(
        map.iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<_>>(),
        vec![("one", 1), ("two", 2), ("three", 3)]
    );

    // Iteration respects insertion order
    for (k, v) in &map {
        println!("{k}: {v}");
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the linked hash map example through its entry point.
    #[test]
    fn test_linked_hash_map_example_entry_point() {
        main();
    }
}
