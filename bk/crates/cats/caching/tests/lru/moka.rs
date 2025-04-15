// ANCHOR: example
use std::time::Duration;

use moka::sync::Cache;

/// Demonstrates the basic usage of the `moka` crate for creating a
/// synchronous cache.
///
/// It shows how to insert, retrieve, and invalidate entries in the cache.
fn main() {
    // Create a cache with a maximum of 100 entries.
    // Entries will be evicted after 10 minutes of not being accessed.
    let cache = Cache::builder()
        .max_capacity(100)
        .time_to_idle(Duration::from_secs(600))
        .build();

    // Insert a key-value pair.
    cache.insert("key1", "value1");

    // Retrieve a value.
    match cache.get("key1") {
        Some(value) => println!("Retrieved value: {}", value),
        None => println!("Value not found"),
    }

    // Remove an entry.
    cache.invalidate("key1");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review examples in https://docs.rs/crate/moka/0.12.10 ; cover async case (sep. example)?](https://github.com/john-cd/rust_howto/issues/1319)
