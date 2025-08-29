#![allow(dead_code)]
// ANCHOR: example
use slotmap::SecondaryMap;
use slotmap::SlotMap;

fn main() {
    // Create a new `SlotMap`:
    let mut sm = SlotMap::new();

    // Upon insertion, a unique key is generated.
    // It can be used to later access or remove the values.
    // Insertion, removal and access all take O(1) time.
    let key1 = sm.insert("value");

    // The difference between a `BTreeMap` or `HashMap` and a slot map is that
    // the key is generated, always unique,
    // and will only refer to the entity that was inserted.

    let key2 = sm.insert("value");
    // `key1` and `key2` are not the same, even though they contain the same value:
    assert_ne!(key1, key2);

    // Access values by key:
    assert_eq!(sm[key1], "value");
    println!("Value for `key1`: {:?}", sm.get(key1));

    // Remove a value.
    // The keys returned by `slotmap` are versioned.
    // This means that once a key is removed, it stays removed,
    // even if the physical storage inside the `SlotMap` is reused for new
    // elements.
    sm.remove(key1);
    assert!(!sm.contains_key(key1));

    // Try to access the removed value:
    println!("`key1` after removal: {:?}", sm.get(key1));

    // `key2` is not affected.
    assert_eq!(sm.get(key2), Some(&"value"));

    // We can also create (multiple) secondary maps that can map the keys
    // returned by `SlotMap` to other values, to associate additional arbitrary data with
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
