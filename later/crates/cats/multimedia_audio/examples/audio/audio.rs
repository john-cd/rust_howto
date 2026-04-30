#![allow(dead_code)]
// ANCHOR: example
//! Buildable placeholder example.
fn main() {
    println!("Placeholder example.");
}
// ANCHOR_END: example

#[test]
#[ignore = "later"]
fn test() {
    main();
}
// [write](https://github.com/john-cd/rust_howto/issues/806)
