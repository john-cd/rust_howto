#![allow(dead_code)]
// ANCHOR: example
/// This example demonstrates a simple panic.
/// When executed, it will cause the program to terminate abruptly.
fn main() {
    panic!("Crash and burn");
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
