#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to use a custom hash function (FNV) with `HashMap` and
//! `HashSet` for better performance with small keys.
//!
//! **Warning:** FNV is not suitable if there is a risk of denial-of-service
//! attacks. Be certain that your program is not exposed to malicious inputs.
//!
//! Add to your `Cargo.toml`: `fnv = "1.0.7" # Or latest`.

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut fnv_map = HashMap::with_hasher(fnv::FnvBuildHasher::default());
    fnv_map.insert("key1", "value1");
    fnv_map.insert("key2", "value2");

    // Or simpler:
    let mut _map: fnv::FnvHashMap<&str, &str> = fnv::FnvHashMap::default();

    // Similarly for `HashSet`:
    let mut fnv_set = fnv::FnvHashSet::default();
    fnv_set.insert("item1");
    fnv_set.insert("item2");

    // You may also pre-allocate capacity for better performance:
    let mut _map_with_capacity: HashMap<String, String> =
        HashMap::with_capacity(100);
    let mut _set_with_capacity: HashSet<String> = HashSet::with_capacity(100);

    // Or set both capacity and a custom hasher:
    let mut map =
        fnv::FnvHashMap::with_capacity_and_hasher(10, Default::default());
    map.insert(1, "one");
    map.insert(2, "two");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
