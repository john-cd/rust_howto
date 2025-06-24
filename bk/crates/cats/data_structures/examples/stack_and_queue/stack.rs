#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let mut stack = Vec::new();

    stack.push(1); // Push 1 onto the stack.
    stack.push(2); // Push 2 onto the stack.
    stack.push(3); // Push 3 onto the stack.

    println!("Top element: {:?}", stack.last()); // Peek at the top element.

    // `pop` eemoves and returns the top element. Returns None if the stack is
    // empty.
    while let Some(top) = stack.pop() {
        // Pop elements until the stack is empty.
        println!("Popped: {}", top);
    }

    println!("Stack is empty: {}", stack.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
