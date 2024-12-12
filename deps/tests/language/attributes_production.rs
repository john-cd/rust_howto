// ANCHOR: example
//! Crate documentation goes here.
#![warn(unused, missing_debug_implementations, missing_docs, rust_2018_idioms)]
// You may also add `missing_copy_implementations` if desirable.
// It detects potentially-forgotten implementations of Copy for public types.

// `deny` creates an error in case of violation
#![deny(unreachable_pub)]
// Prohibit unsafe blocks / functions
// `forbid` is the same as `deny`, but also forbids changing the lint level
// afterwards
#![forbid(unsafe_code)]

// WARNING: fn dead_code() {}

// ERROR: unsafe fn unsafe_func() {}

// ERROR
// fn unsafe_block() {
//     unsafe {
//     }
// }

/// This is the required documentation for S
/// We had to derive Debug to avoid a warning
#[derive(Debug)]
pub(crate) struct S;

/// Here is the required documentation
/// for the main function.
fn main() {
    let s = S;
    println!("{:?}", s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
