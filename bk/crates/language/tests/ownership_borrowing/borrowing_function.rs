#![allow(unused_assignments, dead_code)]
// ANCHOR: example
//! Demonstrates the concept of borrowing in Rust.

// This struct does not implement `Copy` and has therefore "move semantics":
#[derive(Debug)]
struct MyStruct(bool);

// This function takes an (immutable) reference to `MyStruct`.
// We can read but not modify `s1`.
fn calculate(s: &MyStruct) {
    println!("{s:?}");
}

fn main() {
    let s1 = MyStruct(true);
    // `ref_s1` is an immutable reference to `s1`.
    // We call the action of creating a reference "borrowing".
    let ref_s1 = &s1;

    // We pass the reference to `calculate`:
    calculate(ref_s1);

    // `s` goes out of scope at the end of the `calculate` function.
    // Because this function does not have ownership of what it refers
    // to, `s1` is _not_ dropped and remains valid after borrowing:
    println!("{s1:?}");

    // Immutable references are `Copy`, thus `ref_s1` is also still valid:
    println!("{ref_s1:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
