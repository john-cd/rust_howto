// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fixed-capacity collections from the `heapless` crate.

use heapless::FnvIndexMap;
use heapless::String;
use heapless::Vec;

/// Fills a fixed-capacity vector and handles the error when it is full.
fn main() {
    // `heapless` collections have a fixed capacity determined at compile time.
    // Here we define a fixed-size vector with capacity of 3.
    let mut vec: Vec<u32, 3> = Vec::new();

    vec.push(1)
        .expect("the heapless example invariant should hold");
    vec.push(2)
        .expect("the heapless example invariant should hold");
    vec.push(3)
        .expect("the heapless example invariant should hold");
    // Because they have a fixed size, operations like `push` or `insert` can
    // fail if the collection is full. In that case, `push` returns back the
    // item.
    if let Err(err) = vec.push(4) {
        println!("Error pushing to vector: {err:?}");
    }
    assert_eq!(vec.as_slice(), &[1, 2, 3]);

    println!("Vector: {vec:?}");
    println!("Vector length: {}", vec.len());
    println!("Vector capacity: {}", vec.capacity());

    if let Some(last) = vec.pop() {
        println!("Popped: {last}");
        assert_eq!(last, 3);
    }

    // Declare a fixed-size string with capacity of 16:
    let mut string: String<16> = String::from("Hello");

    assert!(string.push_str(", world!").is_ok());

    println!("String: {string}");
    println!("String length: {}", string.len());
    println!("String capacity: {}", string.capacity());

    // Returns an error, if we exceed the capacity:
    let result = string.push_str(" It is too much!");
    if let Err(err) = result {
        println!("Error pushing to string: {err:?}"); // `err` is simply `()`.
    }
    assert_eq!(string.as_str(), "Hello, world!");

    // Define a fixed-size map (using the `Fnv` hash for performance):
    let mut map: FnvIndexMap<&str, u32, 4> = FnvIndexMap::new();

    map.insert("one", 1)
        .expect("the heapless example invariant should hold");
    map.insert("two", 2)
        .expect("the heapless example invariant should hold");
    map.insert("three", 3)
        .expect("the heapless example invariant should hold");
    map.insert("four", 4)
        .expect("the heapless example invariant should hold");
    // When a key already exists, its position in iteration order stays the
    // same, its value is updated, and the old value is returned in `Some(_)`.
    assert_eq!(map.insert("three", 33), Ok(Some(3)));

    println!("Map: {map:?}");

    if let Some(value) = map.get("two") {
        println!("Value for 'two': {value}");
        assert_eq!(*value, 2);
    }

    let result = map.insert("five", 5);
    if let Err(err) = result {
        println!("Error inserting to map: {err:?}");
    }

    // Iterate over the map:
    for (key, value) in &map {
        println!("Key: {key}, Value: {value}");
    }

    // Clear the map:
    map.clear();
    println!("Map is empty: {}", map.is_empty());
    assert!(map.is_empty());
}
// ANCHOR_END: example

/// Runs the `heapless` collection example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the heapless example through its entry point.
    #[test]
    fn test_heapless_example_entry_point() {
        main();
    }
}
