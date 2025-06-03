// ANCHOR: example
//! This example demonstrates move semantics in Rust.
//!
//! Note that `String` (and all non-`Copy` types) have "move" semantics.

fn main() {
    // The variable `s1` owns the string "hello".
    let s1 = String::from("hello");

    // The value in `s1` is MOVED into `s2`.
    // `s2` now owns the string "hello" and Rust invalidates `s1`.
    // This is NOT a shallow copy or a deep copy.
    let s2 = s1;

    println!("{}, world!", s2);
    // ERROR: println!("{}, world!", s1); // `s1` is invalid.
}
// `s2` gets out of scope here, therefore the String it owns is dropped
// (deallocated). `s1` invalidation earlier prevents a "double free" error,
// where two variables might try to deallocate the same memory when they go out
// of scope.

// Behind the scene: during runtime, the local variable `s1` (on the stack)
// contains a pointer to heap-allocated data (here, the unicode characters of
// the String). During the assignment `let s2 = s1`, the pointer is copied into
// `s2`. The heap data is not copied. Earlier during compilation, the Rust
// compiler made sure that `s1` would not be reused after the move, ensuring
// memory safety.

// ANCHOR_END: example

#[test]
fn test() {
    main();
}
