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
    // Normally, we would get a warning abou using a deprecated function.
    foo();
    // In this case, we used the module-wide `#![allow(deprecated)]` attribute
    // (first line of the example) to suppress the warning.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
