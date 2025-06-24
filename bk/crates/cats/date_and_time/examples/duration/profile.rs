#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to profile the execution time of a function
//! using `Instant` and `Duration`.
//!
//! It defines an `expensive_function` that simulates a time-consuming operation
//! by sleeping for 1 second. The `main` function measures the time taken by
//! `expensive_function` and prints the duration.

use std::thread;
use std::time::Duration;
use std::time::Instant;

fn expensive_function() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
