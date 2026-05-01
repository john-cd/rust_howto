#![allow(dead_code)]
// ANCHOR: example
use arrayvec::ArrayVec;

/// This example demonstrates basic usage of `ArrayVec`.
fn main() {
    // Create an empty `ArrayVec` of capacity 5:
    // The capacity is of type `usize` but is range-limited to `u32::MAX`.
    let mut array = ArrayVec::<_, 5>::new();
    assert_eq!(array.capacity(), 5);

    // Push some elements into the `ArrayVec`:
    array.push(1);
    array.push(2);

    // Try extending from a slice:
    array.try_extend_from_slice(&[3, 4, 5]);
    assert!(array.is_full());
    assert_eq!(&array[..], &[1, 2, 3, 4, 5]);

    // Pushing beyond the capacity will result in a panic:
    // ERROR: array.push(6);

    let overflow = array.try_push(6);
    assert!(overflow.is_err());

    // Access elements:
    for i in 0..array.len() {
        println!("Element at index {}: {}", i, array[i]);
    }
    assert_eq!(&array[..], &[1, 2, 3, 4, 5]);

    // Build from an array:
    let mut array2: ArrayVec<i32, 5> = ArrayVec::from([1, 2, 3, 4, 5]);

    // Pop an element from the `ArrayVec`:
    if let Some(value) = array2.pop() {
        println!("Popped value: {value}"); // 5
    }
    assert_eq!(array2.len(), 4);
    assert!(!array2.is_empty());

    // Insert an element:
    array2.insert(2, 99);
    assert_eq!(&array2[..], &[1, 2, 99, 3, 4]);

    // Remove an element:
    let removed = array2.remove(2);
    assert_eq!(removed, 99);
    assert_eq!(&array2[..], &[1, 2, 3, 4]);

    // Retain elements based on a condition:
    array2.retain(|x| *x % 2 == 0);
    assert_eq!(&array2[..], &[2, 4]);

    // Drain elements:
    let drained: Vec<_> = array2.drain(..).collect();
    assert_eq!(drained, vec![2, 4]);
    assert!(array2.is_empty());

    // Clear the array
    array.clear();
    assert!(array.is_empty());
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
