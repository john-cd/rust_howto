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

    assert_eq!(small_vec.len(), 4);
    // Elements are stored inline (no heap allocation yet)
    assert!(!small_vec.spilled());

    // We can also initialize it via a macro:
    let mut small_vec: SmallVec<[i32; 4]> = smallvec![1, 2, 3, 4];

    // Print the current state of the SmallVec.
    println!("SmallVec (inline): {small_vec:?}");

    // Push beyond the inline capacity, causing a heap allocation.
    small_vec.push(5);

    // The vector has now spilled over to the heap
    assert!(small_vec.spilled());
    // The capacity grows upon spilling (typically doubling the original
    // capacity)
    assert_eq!(small_vec.capacity(), 8);

    // Print the state of the SmallVec after pushing beyond capacity.
    println!("SmallVec (heap-allocated): {small_vec:?}");

    // Access elements using indexing.
    for i in 0..small_vec.len() {
        println!("Element at index {i}: {}", small_vec[i]);
    }
    assert_eq!(small_vec[0], 1);
    assert_eq!(small_vec.last(), Some(&5));

    // Pop an element from the SmallVec.
    let value = small_vec.pop();
    assert_eq!(value, Some(5));

    // Print the state of the SmallVec after popping.
    println!("SmallVec after popping: {small_vec:?}");

    // SmallVec points to a slice, so we can use normal slice indexing and
    // other slice methods.
    small_vec[0] = small_vec[1] + small_vec[2];
    small_vec.sort();

    let expected: SmallVec<[i32; 4]> = smallvec![2, 3, 4, 5];
    assert_eq!(small_vec, expected);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
