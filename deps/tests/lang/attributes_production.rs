// ANCHOR: example
//! Crate documentation
#![warn(
    unused,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    rust_2018_idioms
)]
#![deny(unreachable_pub)] // error if violation
#![forbid(unsafe_code)] // same as `deny` +forbids changing the lint level afterwards

/// This is the required documentation for S
#[derive(Debug, Copy, Clone)]
struct S;

/// Here is the main test function!

fn main() {
    let _ = S;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
