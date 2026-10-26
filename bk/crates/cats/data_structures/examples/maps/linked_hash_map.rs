// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A `LinkedHashMap`, a hash map that remembers insertion order.

use linked_hash_map::LinkedHashMap;

/// Inserts into a `LinkedHashMap` and iterates in insertion order.
fn main() {
    let mut map = LinkedHashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

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

    #[test]
    fn test_main() {
        main();
    }
}
