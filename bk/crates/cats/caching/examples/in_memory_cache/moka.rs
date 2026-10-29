// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Basic usage of the `moka` crate for high-performance concurrent caching.
//!
//! ### Purpose and Architecture of `moka`
//! `moka` is a fast, feature-rich, concurrent caching library for Rust inspired
//! by Java's Caffeine:
//! - **High Concurrency**: Provides lock-free or highly concurrent read/write
//!   operations using lock striping and lock-free data structures.
//! - **TinyLFU Eviction Policy**: Uses TinyLFU (Frequency-based eviction) to
//!   achieve near-optimal hit ratios compared to basic LRU policies.
//! - **TTL & TTI Expiration**: Supports Time-To-Live (TTL) and Time-To-Idle
//!   (TTI) entry expiration policies.
//! - **Synchronous & Asynchronous**: Provides both synchronous
//!   (`moka::sync::Cache`) and async (`moka::future::Cache`) interfaces.
//!
//! `moka` also provides an asynchronous (futures aware) cache in its `future`
//! module.
//!
//! Remember to add `moka = "0.12.10"` or latest to your `Cargo.toml`.

use std::time::Duration;

use moka::sync::Cache;

/// Builds a `moka` cache with a size limit and an idle timeout, then inserts
/// and reads entries.
fn main() {
    // Create a cache with a maximum of 100 entries.
    // Entries will be evicted after 10 minutes of not being accessed.
    //
    // The `sync::Cache` struct provides a synchronous cache.
    let cache = Cache::builder()
        .max_capacity(100)
        .time_to_idle(Duration::from_secs(600))
        .build();

    // Insert a key-value pair:
    cache.insert("key1", "value1");

    // Retrieve a value:
    match cache.get("key1") {
        Some(value) => println!("Retrieved value: {value}"),
        None => println!("Value not found"),
    }
    // Note: Every time `get` is called for an existing key, it creates a clone
    // of the stored value and returns it. If you want to store values that
    // are expensive to clone, wrap them in `std::sync::Arc`.

    // Remove an entry:
    cache.invalidate("key1");

    // To share the same cache across multiple threads, clone it.
    // This is a cheap operation.
    let cache_clone = cache.clone();
    let handle = std::thread::spawn(move || {
        cache_clone.insert("key2", "value2");
    });
    handle.join().expect("worker thread should not panic");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the moka example through its entry point.
    #[test]
    fn test_moka_example_entry_point() {
        main();
    }

    #[test]
    fn insert_get_and_invalidate() {
        let cache = Cache::builder()
            .max_capacity(100)
            .time_to_idle(Duration::from_secs(600))
            .build();
        assert_eq!(cache.get("key1"), None);
        cache.insert("key1", "value1");
        assert_eq!(cache.get("key1"), Some("value1"));
        cache.invalidate("key1");
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn clones_share_the_same_cache_across_threads() {
        let cache: Cache<&str, &str> = Cache::new(10);
        let clone = cache.clone();
        std::thread::spawn(move || clone.insert("key2", "value2"))
            .join()
            .expect("worker thread should not panic");
        assert_eq!(cache.get("key2"), Some("value2"));
    }

    #[test]
    fn capacity_is_bounded() {
        let cache: Cache<u32, u32> = Cache::new(5);
        for i in 0..100 {
            cache.insert(i, i);
        }
        cache.run_pending_tasks();
        assert!(cache.entry_count() <= 5);
    }
}
// Reference docs: <https://docs.rs/crate/moka/0.12.10>
