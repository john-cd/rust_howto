// ANCHOR: example
#![allow(deprecated)] // Removes the warning.

// Deprecated function
#[deprecated(since = "5.2.0", note = "Use bar instead")]
pub fn foo() {
    println!("foo");
}

fn main() {
    foo();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
