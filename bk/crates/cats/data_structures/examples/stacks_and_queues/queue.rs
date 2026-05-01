#![allow(dead_code)]
// ANCHOR: example
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    // You can also use `with_capacity`.

    // Add to the queue:
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // Initialize a queue from an array:
    let mut q2 = VecDeque::from([4, 5]);

    // Append to the first queue:
    queue.append(&mut q2);

    // Peek at the front element:
    println!("Front element: {:?}", queue.front());

    // Dequeue elements from the front, until the queue is empty.
    while let Some(front) = queue.pop_front() {
        println!("Dequeued: {front}");
    }
    // Consider using `pop_front_if` as well.

    println!("Queue is empty: {}", queue.is_empty());

    // `VecDeque` is in fact a double-ended queue that offers `push_front`,
    // `back`, `back_mut`, `pop_back`, `pop_back_if` operations:
    queue.push_front(-1);
    queue.push_front(-2);
    assert_eq!(queue.back(), Some(&-1));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
