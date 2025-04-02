// ANCHOR: example
use crossbeam_queue::ArrayQueue;

/// Demonstrates the basic usage of `ArrayQueue`.
fn main() {
    // Create a new `ArrayQueue` with a capacity of 2.
    let q = ArrayQueue::new(2);
    // Push 'a' onto the queue. This should succeed.
    assert_eq!(q.push('a'), Ok(()));
    // Push 'b' onto the queue. This should also succeed.
    assert_eq!(q.push('b'), Ok(()));
    assert_eq!(q.push('c'), Err('c'));
    assert_eq!(q.pop(), Some('a'));
    println!("{:?}", q.pop());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
