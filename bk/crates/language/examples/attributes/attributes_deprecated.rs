#![allow(dead_code)]
// ANCHOR: example
#![allow(deprecated)]

// Mark a function as deprecated.
// That also works for structs, enums, etc...
#[deprecated(since = "5.2.0", note = "Use `bar` instead")]
pub fn foo() {
    println!("foo");
}

fn main() {
    // Use of a deprecated item.
    // Normally, we would get a warning about using a deprecated function.
    foo();
    // In this case, the module-wide `#![allow(deprecated)]` attribute
    // (first line of the example) suppresses the warning.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
