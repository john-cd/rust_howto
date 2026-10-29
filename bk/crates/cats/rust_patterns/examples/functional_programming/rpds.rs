// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! # RPDS (Rust Persistent Data Structures) Example
//!
//! This example demonstrates the use of the `rpds` crate, which provides
//! purely functional (immutable) data structures with structural sharing for
//! efficiency.

use rpds::HashTrieMap;
use rpds::Vector;

/// Uses persistent vectors, maps and sets, which are never modified in place.
fn main() {
    // Persistent `Vector` with immutable updates:
    let v1: Vector<i32> = Vector::new().push_back(1).push_back(2).push_back(3);

    // Create a new vector with index 1 set to 20:
    let v2 = v1
        .set(1, 20)
        .expect("the rpds example invariant should hold");

    println!("Original v1: {v1}"); // [1, 2, 3]
    println!("Modified v2: {v2}"); // [1, 20, 3]
    println!("v1 is unchanged: {v1}"); // [1, 2, 3]

    // Iterating:
    for item in &v2 {
        println!("Item: {item}");
    }

    // Persistent map implemented with a hash array mapped trie:
    let m1: HashTrieMap<String, i32> = HashTrieMap::new();
    let m2 = m1.insert("one".to_owned(), 1).insert("two".to_owned(), 2);
    let m3 = m2.insert("three".to_owned(), 3);

    println!("m2 size: {}", m2.size()); // 2
    println!("m3 size: {}", m3.size()); // 3

    // Lookup:
    println!("Value for 'two': {:?}", m3.get(&"two".to_owned())); // Some(2)

    // Removal (creates a new map):
    let m4 = m3.remove(&"one".to_owned());
    println!("m4 after removal: {m4}");
    println!("m3 is unchanged: {m3}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rpds example through its entry point.
    #[test]
    fn test_rpds_example_entry_point() {
        main();
    }

    /// Updating a persistent vector leaves the original unchanged.
    #[test]
    fn vector_is_persistent() {
        let v1: Vector<i32> =
            Vector::new().push_back(1).push_back(2).push_back(3);
        let v2 = v1.set(1, 20).expect("index in range");
        assert_eq!(v1.iter().copied().collect::<Vec<_>>(), [1, 2, 3]);
        assert_eq!(v2.iter().copied().collect::<Vec<_>>(), [1, 20, 3]);
        assert!(v1.set(3, 0).is_none());
    }

    /// Inserting and removing in a persistent map leaves earlier versions
    /// unchanged.
    #[test]
    fn map_is_persistent() {
        let m2 = HashTrieMap::<String, i32>::new()
            .insert("one".to_owned(), 1)
            .insert("two".to_owned(), 2);
        let m3 = m2.insert("three".to_owned(), 3);
        assert_eq!((m2.size(), m3.size()), (2, 3));
        assert_eq!(m3.get("two"), Some(&2));
        let m4 = m3.remove("one");
        assert_eq!(m4.size(), 2);
        assert_eq!(m3.get("one"), Some(&1));
        assert_eq!(m4.get("one"), None);
    }
}
