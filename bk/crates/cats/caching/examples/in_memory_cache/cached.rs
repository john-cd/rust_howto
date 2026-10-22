#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates two key uses of the `cached` crate:
//! - Simple function memoization with `#[cached]`,
//! - Advanced caching with custom configuration options.
//!
//! Remember to add `cached = "0.55.1"` or latest to your Cargo.toml.

use cached::proc_macro::cached;

/// Cached function with default configuration. Uses the unbounded cache.
/// By default, the cache's name will be the function's name in all caps.
#[cached]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

/// Cached function with custom time-to-live and max capacity.
#[cached(
    // Max cache size:
    size = 50,
    // Set a time-to-live (TTL) for cached entries:
    time = 60,
    // Specify whether to refresh the TTL on cache hits:
    time_refresh = true,
 )]
fn fetch_data(key: usize) -> String {
    println!("Expensive data fetching simulated here.");
    format!("Data for key {key}")
}
// You can also specify the cache name and type, the way it is created, the key
// type, execution synchronization options, handling of `Result` and `Option`
// return values, etc.

/// Calls the memoized functions and shows that repeated calls use the cache.
fn main() {
    // First call computes and caches:
    println!("Fibonacci of 10: {}", fibonacci(10));

    // Subsequent calls retrieve from cache:
    println!("Cached Fibonacci of 10: {}", fibonacci(10));

    // Check the underlying cache:
    {
        use cached::Cached as _;
        let cache = FIBONACCI.lock();
        println!("misses: {:?}", cache.cache_misses());
        assert_eq!(cache.cache_misses(), Some(11));
        // Make sure the lock is dropped.
    }

    // Example with customized cache:
    let first_fetch = fetch_data(123);
    let cached_fetch = fetch_data(123);
    assert_eq!(first_fetch, "Data for key 123");
    assert_eq!(cached_fetch, first_fetch);
    println!("User data: {first_fetch}");
    println!("Cached user data: {cached_fetch}");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    /// Memoized functions return the expected values on repeated calls.
    #[test]
    fn cached_functions_return_the_same_values_on_repeated_calls() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fetch_data(123), "Data for key 123");
        assert_eq!(fetch_data(123), "Data for key 123");
    }
}
// [review further](https://github.com/john-cd/rust_howto/issues/1354)
// <https://docs.rs/cached/latest/cached/proc_macro/index.html>
// <https://github.com/jaemk/cached/tree/master/examples>
