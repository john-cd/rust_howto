// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! An `IndexMap`, a hash map that preserves insertion order.
//!
//! An `IndexMap` is a hash map that remembers insertion order and lets you
//! access entries by position. Use it when the order of keys matters, such as
//! in configuration files.

use indexmap::IndexMap;

/// Inserts into an `IndexMap` and iterates in insertion order.
fn main() {
    // Create an IndexMap:
    let mut map = IndexMap::new();

    // Insert elements:
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    assert_eq!(map.len(), 3);
    assert_eq!(
        map.get_index(1).map(|(key, value)| (*key, *value)),
        Some(("b", 2))
    );

    // Iterate in insertion order:
    println!("Iterating over `IndexMap` in insertion order:");
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Access elements by index:
    if let Some((key, value)) = map.get_index(1) {
        println!("Element at index 1: {key}: {value}");
    }

    // Use the `entry` API:
    map.entry("d").or_insert(4);
    map.entry("a").or_insert(10); // This won't change "a" because it already exists.
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("d"), Some(&4));
    assert_eq!(
        map.get_index(3).map(|(key, value)| (*key, *value)),
        Some(("d", 4))
    );
    println!("IndexMap after using entry API:");
    for (key, value) in &map {
        println!("{key}: {value}");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the indexmap example through its entry point.
    #[test]
    fn test_indexmap_example_entry_point() {
        main();
    }
}
