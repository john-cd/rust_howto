// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A `SkipMap`, an ordered map built on a skip list.
//!
//! A skip list keeps sorted elements in layered linked lists, where upper
//! layers skip ahead, giving O(log n) search without rebalancing like a tree.
//! Databases use it for in-memory sorted indexes, and it is easier to make
//! concurrent than a tree.

use skiplist::SkipMap;

/// Inserts into a `SkipMap` and iterates in key order.
fn main() {
    let mut map: SkipMap<i32, &str, 16> = SkipMap::new();

    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&2), Some(&"two"));

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

    /// Runs the skiplist example through its entry point.
    #[test]
    fn test_skiplist_example_entry_point() {
        main();
    }
}
