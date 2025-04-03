#![allow(clippy::useless_vec)]
// ANCHOR: example

/// This example demonstrates the difference between `iter()` and `into_iter()`.
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for `Vec<i32>` yields `&i32`, which we
    // destructured via the `&x` pattern.
    // It only borrows `vec1` and its elements,
    // so they can be used again.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // `into_iter()` for vecs yields `i32`. No destructuring is required.
    // `into_iter()` does move `vec2` and its elements, so they cannot be
    // used again
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
