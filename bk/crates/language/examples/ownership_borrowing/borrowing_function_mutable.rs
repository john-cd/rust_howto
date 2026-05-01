#![allow(dead_code)]
// ANCHOR: example
/// This function takes a mutable reference `&mut` to a `String`
/// and appends ", world" to it.
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modifies the string in place.
    println!("{some_string}");
}

fn main() {
    let mut s = String::from("hello"); // The `mut` keyword is required.

    // Create a mutable reference to `s`:
    let ref_mut1 = &mut s;

    // We can pass the string by mutable reference to a function and modify it.
    change(ref_mut1);
    // While the `String` type has "move semantics", the function does not
    // consume the string, since the mutable reference does not own it.

    // We cannot create other mutable or immutable references while the
    // exclusive reference is in use:
    // let ref_mut2 = &mut s;
    // ERROR: cannot borrow `s` as mutable more than once at a time.
    // let ref1 = &s;
    // ERROR: cannot borrow `s` as immutable because it is also borrowed as
    // mutable.

    println!("{ref_mut1}");

    // The original data can be borrowed again only after the mutable reference
    // has been used for the last time.
    let _ref2 = &s;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
