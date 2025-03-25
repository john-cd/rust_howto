// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use cached::proc_macro::cached;

// // This example demonstrates two key uses of the `cached` crate:
// // - Simple function memoization with `#[cached]`,
// // - Advanced caching with custom configuration options.
// // Remember to add cached = "0.55.1" or latest to your Cargo.toml.

// // Cached function with default configuration
// #[cached]
// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         return n;
//     }
//     fibonacci(n - 1) + fibonacci(n - 2)
// }

// // Cached function with custom time-to-live and max capacity
// #[cached(
//     time = 300, // Sets a time-to-live (TTL) of 300 seconds for cached
// entries.     time_refresh = true, //  Specify whether to refresh the TTL on
// cache hits.     key = "String", // Specify what type to use for the cache
// key.     convert = r#"{ String::from(key) }"#, // Defines how to convert the
// function's input (key: &str) into the cache key (here String). )]
// fn fetch_user_data(key: &str) -> String {
//     // Simulated expensive data fetching
//     format!("User data for {}", key)
// }

// fn main() {
//     // First call computes and caches
//     println!("Fibonacci 10: {}", fibonacci(10));

//     // Subsequent calls retrieve from cache
//     println!("Cached Fibonacci 10: {}", fibonacci(10));

//     // User data caching
//     println!("User data: {}", fetch_user_data("user123"));
//     println!("Cached user data: {}", fetch_user_data("user123"));
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish example](https://github.com/john-cd/rust_howto/issues/1320)
// // review examples in https://docs.rs/cached/latest/cached/proc_macro/index.html
