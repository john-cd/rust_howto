#![allow(dead_code)]
// ANCHOR: example
use crossbeam_utils::atomic::AtomicCell;

/// Demonstrates the use of `AtomicCell` from the `crossbeam-utils` crate.
fn main() {
    // Create a new AtomicCell with an initial value of 7.
    let a = AtomicCell::new(7);

    // Loads a value from the atomic cell.
    assert_eq!(a.load(), 7);
    // Stores a value into the atomic cell.
    a.store(8);
    assert_eq!(a.load(), 8);
    // Stores val into the atomic cell and returns the previous value.
    assert_eq!(a.swap(9), 8);
    assert_eq!(a.load(), 9);

    // Extract the inner value from the AtomicCell.
    // Consumes the atomic and returns the contained value.
    let v = a.into_inner();
    assert_eq!(v, 9);
    println!("{}", v);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
