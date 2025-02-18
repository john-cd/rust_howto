// ANCHOR_END: example
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // BinaryHeap represents a priority queue where elements can be efficiently
    // inserted and the maximum or minimum element can be quickly accessed.
    // By default, BinaryHeap is a max-heap, meaning the largest element is
    // always at the top.
    let mut heap = BinaryHeap::new();

    // Push elements into the BinaryHeap
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);
    heap.push(9);

    assert_eq!(heap.peek(), Some(&9));

    // We can iterate over the items in the heap, although they are
    // returned in a random order.
    for x in &heap {
        println!("{x}");
    }

    // Pop elements from the BinaryHeap
    while let Some(value) = heap.pop() {
        println!("{}", value);
    }
    // Since BinaryHeap is a max-heap by default, the elements are printed in
    // descending order: 9, 5, 4, 3, 1, 1.

    // The heap should now be empty.
    assert!(heap.is_empty());

    // If you need a min-heap instead of a max-heap,
    // use std::cmp::Reverse to invert the order.
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(3));
    heap.push(Reverse(1));
    heap.push(Reverse(4));
    // Pop elements from the BinaryHeap
    while let Some(Reverse(value)) = heap.pop() {
        println!("{}", value);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
