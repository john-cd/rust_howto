// ANCHOR: example
use crossbeam_utils::atomic::AtomicCell;

/// Demonstrates the use of `AtomicCell` from the `crossbeam-utils` crate.
fn main() {
    // Create a new AtomicCell with an initial value of 7.
    let a = AtomicCell::new(7);

    // Extract the inner value from the AtomicCell.
    let v = a.into_inner();
    assert_eq!(v, 7);
    println!("{}", v);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
