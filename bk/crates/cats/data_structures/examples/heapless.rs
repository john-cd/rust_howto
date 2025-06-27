// ANCHOR: example
//! This example demonstrates the usage of `heapless` collections in Rust.
//!
//! `heapless` provides fixed-size, stack-allocated data structures, which are
//! useful when you know the maximum size of your data at compile time and need
//! to avoid dynamic memory allocation (and potential capacity errors). This is
//! particularly beneficial in embedded systems or performance-critical
//! applications where dynamic allocation can be unpredictable or undesirable.
//!
//! Choosing `heapless` is a trade-off: you gain performance and determinism but
//! lose the flexibility of dynamic resizing.

use heapless::FnvIndexMap;
use heapless::String;
use heapless::Vec;

fn main() -> anyhow::Result<()> {
    // `heapless`` collections have a fixed, compile-time determined capacity.
    // Here we define a fixed-size vector with capacity of 5.
    let mut vec: Vec<u32, 5> = Vec::new();

    // Because they have a fixed size, operations like `push` or `insert` can
    // fail if the collection is full.
    vec.push(1).unwrap();
    vec.push(2).unwrap();
    vec.push(3).unwrap();

    println!("Vector: {vec:?}");
    println!("Vector length: {}", vec.len());
    println!("Vector capacity: {}", vec.capacity());

    if let Some(last) = vec.pop() {
        println!("Popped: {last}");
    }

    // Fixed-size string with capacity of 16:
    let mut string: String<16> = String::from("Hello");

    assert!(string.push_str(", world!").is_ok());

    println!("String: {string}");
    println!("String length: {}", string.len());
    println!("String capacity: {}", string.capacity());

    // Error if we exceed the capacity:
    let result = string.push_str(" It is too much!");
    if let Err(err) = result {
        println!("Error pushing to string: {err:?}"); // err is ()
    }

    // Fixed-size map (using Fnv hash for performance)
    let mut map: FnvIndexMap<&str, u32, 4> = FnvIndexMap::new();

    map.insert("one", 1).unwrap();
    map.insert("two", 2).unwrap();
    map.insert("three", 3).unwrap();
    map.insert("four", 4).unwrap();

    println!("Map: {map:?}");

    if let Some(value) = map.get("two") {
        println!("Value for 'two': {value}");
    }

    let result = map.insert("five", 5);
    if let Err(err) = result {
        println!("Error inserting to map: {err:?}");
    }

    // Iterating over the map:
    for (key, value) in &map {
        println!("Key: {key}, Value: {value}");
    }

    // Clearing the map:
    map.clear();
    println!("Map is empty: {}", map.is_empty());

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
