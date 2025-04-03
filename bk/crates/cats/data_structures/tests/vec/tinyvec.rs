// ANCHOR: example
use tinyvec::TinyVec;

/// Demonstrates the usage of `TinyVec`, a vector-like data structure that
/// can store elements inline up to a certain capacity, and then falls back
/// to heap allocation for larger sizes.
fn main() {
    // Create a TinyVec with an inline capacity of 4 i32 elements.
    let mut tiny_vec: TinyVec<[i32; 4]> = TinyVec::new();

    // Push some elements into the TinyVec.
    tiny_vec.push(1);
    tiny_vec.push(2);
    tiny_vec.push(3);
    tiny_vec.push(4);

    // Print the current state of the TinyVec
    println!("TinyVec (inline): {:?}", tiny_vec);

    // Push beyond the inline capacity, which will cause a heap allocation.
    tiny_vec.push(5);

    // Print the state of the TinyVec after pushing beyond capacity.
    println!("TinyVec (heap-allocated): {:?}", tiny_vec);

    // Access elements using indexing.
    for i in 0..tiny_vec.len() {
        println!("Element at index {}: {}", i, tiny_vec[i]);
    }

    // Pop an element from the TinyVec.
    if let Some(value) = tiny_vec.pop() {
        println!("Popped value: {}", value);
    }

    // Print the state of the TinyVec after popping
    println!("TinyVec after popping: {:?}", tiny_vec);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
