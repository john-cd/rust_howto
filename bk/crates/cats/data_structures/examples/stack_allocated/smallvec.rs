#![allow(dead_code)]
// ANCHOR: example
use smallvec::SmallVec;
use smallvec::smallvec;

fn main() {
    // Create a SmallVec with a small inline capacity of 4.
    // This means that the first 4 elements will be stored directly within the
    // `SmallVec` struct, avoiding heap allocation.
    let mut small_vec: SmallVec<[i32; 4]> = SmallVec::new();

    // Push some elements into the SmallVec.
    small_vec.push(1);
    small_vec.push(2);
    small_vec.push(3);
    small_vec.push(4);

    // We can also initialize it via a macro:
    let mut small_vec: SmallVec<[i32; 4]> = smallvec![1, 2, 3, 4];

    // Print the current state of the SmallVec.
    println!("SmallVec (inline): {small_vec:?}");

    // Push beyond the inline capacity, causing a heap allocation.
    small_vec.push(5);

    // Print the state of the SmallVec after pushing beyond capacity.
    println!("SmallVec (heap-allocated): {small_vec:?}");

    // Access elements using indexing.
    for i in 0..small_vec.len() {
        println!("Element at index {i}: {}", small_vec[i]);
    }

    // Pop an element from the SmallVec.
    if let Some(value) = small_vec.pop() {
        println!("Popped value: {value}");
    }

    // Print the state of the SmallVec after popping.
    println!("SmallVec after popping: {small_vec:?}");

    // SmallVec points to a slice, so we can use normal slice indexing and
    // other methods to access its contents.
    small_vec[0] = small_vec[1] + small_vec[2];
    small_vec.sort();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO review
