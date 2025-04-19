// ANCHOR: example
//! This example demonstrates the basic usage of the `moka` crate for creating
//! a concurrent cache.
//!
//! `moka` is inspired by the 'Caffeine' library for Java.
//!
//! Remember to add `moka = "0.12.10"` or latest to your `Cargo.toml`.
//!
//! `moka` also provides an asynchronous (futures aware) cache in its `future`
//! module.

use std::time::Duration;

use moka::sync::Cache;

fn main() {
    // Create a cache with a maximum of 100 entries.
    // Entries will be evicted after 10 minutes of not being accessed.
    //
    // The `sync::Cache` struct provides a synchronous cache.
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
    // Note: Every time get is called for an existing key, it creates a clone of
    // the stored value and returns it. If you want to store values that
    // will be expensive to clone, wrap them in `std::sync::Arc`.

    // Remove an entry.
    cache.invalidate("key1");

    // To share the same cache across multiple threads, clone it.
    // This is a cheap operation.
    let cache_clone = cache.clone();
    let handle = std::thread::spawn(move || {
        cache_clone.insert("key2", "value2");
    });
    handle.join().unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// See also examples in https://docs.rs/crate/moka/0.12.10
