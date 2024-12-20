// ANCHOR: example
#![allow(deprecated)]

// Mark a function as deprecated
// That also works for structs, enums, etc...
#[deprecated(since = "5.2.0", note = "Use bar instead")]
pub fn foo() {
    println!("foo");
}

fn main() {
    // Use of a deprecated item
    foo();
    // Normally we would get a warning.
    // In this case, we used the module-wide #![allow(deprecated)] attribute
    // (first line above) to suppress it.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
