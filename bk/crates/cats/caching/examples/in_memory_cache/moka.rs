// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Uses a synchronous Moka cache with idle expiration and a fallible loader.

use std::sync::Barrier;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use moka::sync::Cache;

/// Builds a cache, reads an entry, then demonstrates retrying a failed load.
///
/// # Panics
///
/// Panics if the always-successful retry fails or a worker panics.
fn main() {
    // Target at most 100 entries and expire an entry after 10 minutes idle.
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

    let (values, loads) = retry_and_share_load();
    println!("Loaded {values:?} with {loads} successful initializer call");
}

/// Retries a failed initialization and reports the values and initialization count.
///
/// # Panics
///
/// Panics if the fixed, successful loader returns an error or a worker panics.
fn retry_and_share_load() -> ([&'static str; 2], usize) {
    let cache = Cache::new(10);
    let failed = cache.try_get_with("resource", || Err::<&str, _>("temporary failure"));
    assert!(failed.is_err());

    let ready = Barrier::new(2);
    let loads = AtomicUsize::new(0);
    let load = || {
        ready.wait();
        cache
            .try_get_with("resource", || {
                loads.fetch_add(1, Ordering::Relaxed);
                Ok::<_, &str>("loaded")
            })
            .expect("the retry loader always succeeds")
    };
    let values = thread::scope(|scope| {
        let worker = scope.spawn(load);
        [load(), worker.join().expect("the loader should not panic")]
    });
    (values, loads.load(Ordering::Relaxed))
}
// ANCHOR_END: example

/// Runs the idle-expiration and fallible-loader examples.
///
/// # Panics
///
/// Panics if a demonstration worker fails.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the moka example through its entry point.
    #[test]
    fn test_moka_example_entry_point() {
        main();
    }

    /// Inserts get and invalidate.
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

    /// Verifies that the clones share the same cache across threads.
    #[test]
    fn clones_share_the_same_cache_across_threads() {
        let cache: Cache<&str, &str> = Cache::new(10);
        let clone = cache.clone();
        std::thread::spawn(move || clone.insert("key2", "value2"))
            .join()
            .expect("worker thread should not panic");
        assert_eq!(cache.get("key2"), Some("value2"));
    }

    /// Retries a failed load and initializes the shared key only once.
    #[test]
    fn retry_load_is_shared() {
        assert_eq!(retry_and_share_load(), (["loaded", "loaded"], 1));
    }

    /// Observes eviction after the cache processes pending maintenance.
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
