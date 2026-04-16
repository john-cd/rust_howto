#![allow(dead_code)]
// ANCHOR: example
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // Declare a `BinaryHeap<i32>`.
    // Type inference lets us omit an explicit type signature.
    let mut heap = BinaryHeap::new();

    // Push elements into the `BinaryHeap`.
    // It is O(1) in average, when pushing elements that are not already in any
    // sorted pattern.
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);
    heap.push(9);

    // Return (a reference to) the greatest item in the binary heap,
    // or `None` if it is empty. Its cost is O(1) in the worst case.
    assert_eq!(heap.peek(), Some(&9));

    // We can iterate over the items in the heap, although they are
    // returned in a random order:
    println!("Iteration:");
    for x in &heap {
        println!("{x}");
    }

    // Since `BinaryHeap` is a "max-heap" by default, `pop` removes the greatest
    // item from the binary heap and returns it, or `None` if it is empty.
    // O(log(n)) in the worst case.
    println!("Pop elements:");
    while let Some(value) = heap.pop() {
        println!("{value}");
    }
    // The elements are printed in descending order: 9, 5, 4, 3, 1, 1.

    // The heap should now be empty:
    assert!(heap.is_empty());

    // If we need a "min-heap" instead of a "max-heap",
    // use `std::cmp::Reverse` to invert the order.
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(3));
    heap.push(Reverse(1));
    heap.push(Reverse(4));

    // Pop elements from the `BinaryHeap`:
    println!("Pop in reverse order:");
    while let Some(Reverse(value)) = heap.pop() {
        println!("{value}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
