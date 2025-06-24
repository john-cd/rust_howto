#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the usage of `SlotMap` and `SecondaryMap` from the `slotmap`
//! crate.
//!
//! This example showcases how to create, insert, access, and remove elements
//! from a `SlotMap`, and how to use a `SecondaryMap` to associate additional
//! data with the elements in the `SlotMap`.
use slotmap::SecondaryMap;
use slotmap::SlotMap;

fn main() {
    // Create a new SlotMap.
    let mut sm = SlotMap::new();

    // Upon insertion, a unique key is generated.
    // It can be used to later access or remove the values.
    // Insertion, removal and access all take O(1) time.
    let key1 = sm.insert("value");

    // The difference between a `BTreeMap` or `HashMap` and a slot map is that
    // the key is generated, always unique,
    // and will only refer to the value that was inserted.
    let key2 = sm.insert("value");
    assert_ne!(key1, key2);

    // Access values by key.
    assert_eq!(sm[key1], "value");
    println!("Value for `key1`: {:?}", sm.get(key1));

    // Remove a value
    // The keys returned by slotmap are versioned.
    // This means that once a key is removed, it stays removed,
    // even if the physical storage inside the slotmap is reused for new
    // elements.
    sm.remove(key1);
    assert!(!sm.contains_key(key1));

    // Try to access the removed value.
    println!("Key1 after removal: {:?}", sm.get(key1));

    assert_eq!(sm.get(key2), Some(&"value"));

    // You can also create (multiple) secondary maps that can map the keys
    // returned by SlotMap to other values, to associate arbitrary data with
    // objects stored in slot maps.
    let mut sec = SecondaryMap::new();
    // Insert a value into the secondary map, associated with `key2`.
    sec.insert(key2, "secondary");

    for (key, val) in sm {
        println!("In slotmap: {}; in secondary map: {}", val, sec[key]);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
