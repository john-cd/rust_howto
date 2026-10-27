#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the basic usage of the `LruCache` from the `lru`
//! crate. It shows how to create a cache, insert key-value pairs, access
//! entries, and observe the eviction behavior of the LRU (Least Recently Used)
//! cache.

use std::num::NonZeroUsize;

use lru::LruCache;

/// Fills an LRU cache beyond its capacity and shows which entries are evicted.
fn main() {
    // Create an LRU cache with a capacity of 3:
    let mut cache = LruCache::new(
        NonZeroUsize::new(3).expect("cache capacity should be nonzero"),
    );

    // Insert some key-value pairs into the cache:
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");

    // Access some entries:
    println!("a) value for key 1: {:?}", cache.get(&1)); // Should print `Some("one")`.
    assert_eq!(
        *cache
            .get(&2)
            .expect("expected example value should be present"),
        "two"
    );
    assert_eq!(cache.get(&3), Some(&"three"));
    assert!(cache.get(&4).is_none());

    // Insert another entry, causing the least recently used entry to be
    // evicted.
    cache.put(4, "four");

    // The cache now contains keys 2, 3, and 4.
    println!("b) value for key 1: {:?}", cache.get(&1)); // Should print "None".
    println!("b) value for key 3: {:?}", cache.get(&3)); // Some("three").
    println!("b) value for key 4: {:?}", cache.get(&4)); // Some("four").

    // Insert another entry, causing another eviction.
    cache.put(5, "five");

    // The cache now contains keys 3, 4, and 5.
    println!("Value for key 2: {:?}", cache.get(&2)); // None.
    println!("Value for key 4: {:?}", cache.get(&4)); // Some("four").
    println!("Value for key 5: {:?}", cache.get(&5)); // Some("five").

    {
        // Returns a mutable reference to the value of the key in the cache or
        // `None`, if it is not present in the cache.
        // Moves the key to the head of the LRU list, if it exists.
        let v = cache
            .get_mut(&5)
            .expect("expected example value should be present");
        *v = "new value";
        assert_eq!(cache.get_mut(&5), Some(&mut "new value"));

        assert_eq!(cache.get_mut(&6), None);
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the lru example through its entry point.
    #[test]
    fn test_lru_example_entry_point() {
        main();
    }
}
