// ANCHOR: example
//! A slice is a view into a contiguous sequence of elements in a collection.
//! It does not own the data it points to.

/// Create slices from an array:
fn slice_from_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // Create a slice referencing elements at indices 1, 2, and 3:
    let slice: &[i32] = &array[1..4];
    println!("{:?}", slice); // Output: [2, 3, 4].

    // `[..]` refers to the entire collection.
    let all: &[i32] = &array[..];
    println!("Entire array: {:?}", all);

    // You can also coerce an array to a slice:
    let arr_slice: &[i32] = &[10, 20];
    println!("{:?}", arr_slice);
}

/// Create slices from a `Vec`:
fn slice_from_vector() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    let slice: &[i32] = &vector[1..4]; // [2, 3, 4].
    println!("{:?}", slice);

    // Mutable slice:
    let mutable_slice: &mut [i32] = &mut vector[2..];
    mutable_slice[0] = 10; // Modifies the original vector.
    println!("{:?}", mutable_slice); // Output: [1, 2, 10, 4, 5].
}

fn main() {
    slice_from_array();
    slice_from_vector();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
