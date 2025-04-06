// ANCHOR: example
use arrayvec::ArrayVec;

/// This example demonstrates basic usage of `ArrayVec`.
///
/// `ArrayVec` is a vector backed by a fixed-size array.
///
/// The capacity is of type `usize` but is range-limited to `u32::MAX`.
/// It offers a simple API but also dereferences to a slice, so that the full
/// slice API is available.
fn main() {
    let mut array = ArrayVec::<_, 2>::new();
    assert_eq!(array.capacity(), 2);
    // Push some elements into the `ArrayVec`:
    array.push(1);
    array.push(2);
    assert!(array.is_full());
    // Trying to push beyond the capacity will result in a panic.
    // ERROR: array.push(3);
    let overflow = array.try_push(3);
    assert!(overflow.is_err());
    // Access elements:
    for i in 0..array.len() {
        println!("Element at index {}: {}", i, array[i]);
    }
    assert_eq!(&array[..], &[1, 2]);

    let mut array2: ArrayVec<i32, 3> = ArrayVec::from([1, 2, 3]);
    // Pop an element from the `ArrayVec`:
    if let Some(value) = array2.pop() {
        println!("Popped value: {}", value);
    }
    assert_eq!(array2.len(), 2);
    assert!(!array2.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
