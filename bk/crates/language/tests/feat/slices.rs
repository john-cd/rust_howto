// ANCHOR: example
//! A slice is a view into a contiguous sequence of elements in a collection.
//! It does not own the data it points to.
//!
//! Slices are created by referencing a range of elements within a collection.
//! The range is specified using the `[start..end]` syntax. `start` is the index
//! of the first element to include, and `end` is the index of the element
//! _after_ the last one to include.

/// Slices from an array.
fn array() {
    let array = [1, 2, 3, 4, 5];
    // Create a slice referencing elements at indices 1, 2, and 3
    let slice = &array[1..4];
    println!("{:?}", slice); // Output: [2, 3, 4]
    // `[..]` refers to the entire collection.
    let all = &array[..];
    println!("Entire array: {:?}", all);
}

/// Slices from `Vec`.
fn vectors() {
    let mut my_vector = vec![1, 2, 3, 4, 5];
    let slice = &my_vector[1..4]; // [2, 3, 4]
    println!("{:?}", slice);

    // Mutable slice
    let mutable_slice = &mut my_vector[2..];
    mutable_slice[0] = 10; // Modifies the original vector

    println!("{:?}", my_vector); // Output: [1, 2, 10, 4, 5]
}

/// String slices.
fn string() {
    let s = String::from("hello world");
    let hello: &str = &s[0..5]; // or &s[..5];
    let world = &s[6..11]; // or &s[6..];
    println!("{}", hello);
    println!("{}", world);
}

fn main() {
    array();
    vectors();
    string();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
