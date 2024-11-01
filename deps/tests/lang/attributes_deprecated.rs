// ANCHOR: example
#![allow(deprecated)] // Removes the warning.

#[deprecated(since = "5.2.0", note = "use bar instead")]
pub fn foo() {}

fn main() {
    foo();
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
