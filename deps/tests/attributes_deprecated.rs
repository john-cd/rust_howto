#![allow(deprecated)] // Removes the warning.

#[deprecated(since = "5.2.0", note = "use bar instead")]
pub fn foo() {}

#[test]
fn test() {
    foo();
}
