// ANCHOR: example
// A double-ended queue implemented with a growable ring buffer.
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back(1); // Enqueue 1
    queue.push_back(2); // Enqueue 2
    queue.push_back(3); // Enqueue 3

    println!("Front element: {:?}", queue.front()); // Peek at the front element

    while let Some(front) = queue.pop_front() {
        // Dequeue elements until the queue is empty
        println!("Dequeued: {}", front);
    }

    println!("Queue is empty: {}", queue.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
