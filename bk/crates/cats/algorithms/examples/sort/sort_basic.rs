#![allow(dead_code)]
// ANCHOR: example
/// This example demonstrates the basic usage of the `sort` method on a vector.
///
/// The `sort` method sorts the elements of the vector in ascending order.
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];

    // Sort the vector in ascending order.
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{vec:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
