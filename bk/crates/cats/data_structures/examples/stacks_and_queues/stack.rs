#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // Initialize a stack with a minimum capacity.
    // The stack will be able to hold at least 5 elements without reallocating:
    let mut stack = Vec::with_capacity(5);
    assert_eq!(stack.capacity(), 5);
    assert_eq!(stack.len(), 0);

    // Append elements to the back of the `Vec` / top of the stack:
    stack.push(1);
    stack.push(22);
    stack.push(333);

    // Peek at the top (last) element of the stack, or `None` if it is empty
    println!("Top element: {:?}", stack.last());
    assert_eq!(stack.last(), Some(&333));

    // Remove and return the top (last) element if the predicate returns true,
    // or `None` if the predicate returns false or the vector is empty.
    assert_eq!(stack.pop_if(|x: &mut i32| *x % 2 == 0), None);

    // `pop` removes and returns the top (last) element.
    // It returns `None` if the stack is empty.
    while let Some(top) = stack.pop() {
        // Pop elements until the stack is empty.
        println!("Popped: {top}");
    }

    println!("Stack is empty: {}", stack.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
