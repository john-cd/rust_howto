#![allow(dead_code)]
// ANCHOR: example
use std::collections::HashMap;

/// `derive` is a powerful tool in Rust that allows you to automatically
/// implement certain traits for your structs and enums.
///
/// Here we derive several common traits for a simple struct `S` that wraps an
/// `i32`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

fn simple_tests() {
    let zero = S::default(); // Courtesy of `Default`.
    println!("Debug formatting: {:?}", zero); // Courtesy of `Debug`.
    dbg!(&zero);
    let one = S(1);
    let also_one = one; // Courtesy of `Clone`, `Copy`.
    println!("Equality is implemented. S(1) == S(1): {}", one == also_one); // Courtesy of `PartialEq`, `Eq`.
    println!("Ordering is implemented. S(1) < S(2): {}", one < S(2)); // Courtesy of `PartialOrd`, `Ord`.
}

// The `Hash` trait enables types to be hashed, which means they can
// be used as keys in hash-based collections like `HashMap` and `HashSet`.
// When you use `#[derive(Hash)]` on a struct, the derived implementation
// will typically hash each field of the struct in sequence and combine these
// hashes  into a single hash value for the entire struct instance.
fn hash() {
    let mut map = HashMap::new();

    let s1 = S(10);
    let s2 = S(20);

    // Use instances of `S` as keys:
    map.insert(s1, "Value for S(10)");
    map.insert(s2, "Value for S(20)");

    // Retrieve a value using an `S` key:
    if let Some(value) = map.get(&S(10)) {
        println!("Found: {}", value); // Output: Found: Value for S(10).
    }
}

fn main() {
    simple_tests();
    hash();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
