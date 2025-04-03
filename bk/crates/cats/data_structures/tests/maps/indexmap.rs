// ANCHOR: example
use indexmap::IndexMap;

/// This example demonstrates the usage of `IndexMap` from the `indexmap` crate.
///
/// `IndexMap` is a hash table that preserves the order of key insertion.
/// It provides methods for accessing elements by index and using the `entry`
/// API.
fn main() {
    // Creating an IndexMap
    let mut map = IndexMap::new();

    // Inserting elements
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Iterating in insertion order
    println!("Iterating over IndexMap in insertion order:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Accessing elements by index
    if let Some((key, value)) = map.get_index(1) {
        println!("Element at index 1: {}: {}", key, value);
    }

    // Using the `entry` API
    map.entry("d").or_insert(4);
    map.entry("a").or_insert(10); // This won't change "a" because it already exists
    println!("IndexMap after using entry API:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
