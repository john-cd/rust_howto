#![allow(dead_code)]
// ANCHOR: example
// Move semantics is not limited to assignments.
// They apply to function parameters as well.

// This function consumes its String parameter...
fn consume(x: String) {
    println!("{}", x);
} // ...then, when `x` goes out of scope, its value is dropped.

// This function consumes, processes, then returns its `String` parameter.
fn consume_and_return(x: String) -> String {
    println!("{}", x);
    x
}

fn main() {
    let s1 = String::from("Rust");
    // s1 is moved into the `consume` function:
    consume(s1);
    // `s1` is no longer available:
    // println!("{}", s1); // ERROR: borrow of moved value: `s1`.

    // You can return the value that has been consumed,
    // if you still need it.
    // Most often, however, it is more convenient to borrow the `String`
    // using references (see below).
    let s2 = String::from("Rust");
    let s3 = consume_and_return(s2);
    println!("{}", s3);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
