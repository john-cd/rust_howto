#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! # Persistent Collections with `imbl`
//!
//! These persistent collections share unchanged structure between related
//! versions. Values stored in them must implement `Clone` when an update needs
//! to copy a shared node.

use imbl::HashMap;
use imbl::HashSet;
use imbl::Vector;
use imbl::hashmap;
use imbl::vector;

/// Uses immutable vectors, maps and sets, which share structure between
/// versions.
fn main() {
    // A vector is a sequence that preserves insertion order.

    let mut v1 = Vector::new();
    // Push values to the back of a vector.
    v1.push_back(1);
    v1.push_back(2);
    v1.push_back(3);

    // Clone the vector, then change the new version.
    let mut v2 = v1.clone();
    v2.push_back(4);

    println!("Original vector: {v1:?}");
    println!("Modified vector: {v2:?}");

    // Create another version with the value at index 1 updated.
    let v3 = v2.update(1, 10);
    assert_eq!(v2.get(1), Some(&2));
    assert_eq!(v3.get(1), Some(&10));
    println!("Updated vector: {v3:?}");

    // Simpler: use the `vector!` macro.
    assert_eq!(5, vector![1, 2, 3, 4, 5].len());

    // Build a hash map with the `hashmap!` macro.
    assert_eq!(
        3,
        hashmap! {
        1 => 11,
        2 => 22,
        3 => 33
        }
        .len()
    );

    // Or construct a hash map with a single mapping.
    let map0 = HashMap::unit("key0", 0);
    assert_eq!(map0.get("key0"), Some(&0));

    // Or use `insert`:
    let mut map1 = HashMap::new();
    map1.insert("key1", 100);
    map1.insert("key2", 200);

    // Clone the map and modify.
    let mut map2 = map1.clone();
    map2.insert("key3", 300);

    println!("Original map: {map1:?}");
    println!("Modified map: {map2:?}");

    // Clone a set to retain both versions after an update.
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);

    // Create a new set with a modification.
    let mut set2 = set1.clone();
    set2.insert(3);

    println!("Original set: {set1:?}");
    println!("Modified set: {set2:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the collection examples without panicking.
    #[test]
    fn test_main() {
        main();
    }
}
