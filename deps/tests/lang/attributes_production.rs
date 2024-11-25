// ANCHOR: example
//! Crate documentation goes here.
#![warn(
    unused,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    rust_2018_idioms
)]
#![deny(unreachable_pub)] // Error if violation
#![forbid(unsafe_code)] // Same as `deny`, but also forbids changing the lint level afterwards

/// This is the required documentation for S
#[derive(Debug, Copy, Clone)]
struct S;

/// Here is the required documentation
/// for the main function.
fn main() {
    let _ = S;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO
