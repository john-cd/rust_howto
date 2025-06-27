#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let mut stack = vec![1, 2, 3];

    // `while let` is a control flow construct that allows you to run a loop as
    // long as a pattern matches. We remove the last element of the vector
    // at each iteration, until the vector is empty and `pop` returns `None`,
    // which stops the loop.
    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
