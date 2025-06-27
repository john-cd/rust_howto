// ANCHOR: example
//! Typical lint attributes for production code.
//!
//! The following is a set of attributes commonly used in production
//! code to enforce good practices and catch potential issues early.

// The `warn` attribute creates compiler warnings in case of violation.
#![warn(unused, missing_debug_implementations, missing_docs, rust_2018_idioms)]
// You may also add `missing_copy_implementations` if desirable.
// It detects potentially-forgotten implementations of `Copy` for public types.

// The `deny` attribute creates an error in case of violation.
#![deny(unreachable_pub)]
// The following prohibits unsafe blocks / functions.
// `forbid` is the same as `deny`, but also forbids changing the lint level
// afterwards.
#![forbid(unsafe_code)]

// Uncomment the following to observe compiler warnings or errors:

// WARNING: fn dead_code() { println!("This function is not used!"); }

// ERROR: unsafe fn unsafe_func() {
// println!("This function is marked as unsafe.");
// }

// ERROR:
// fn contains_a_unsafe_block() {
//     unsafe {
//     }
// }

/// This is the required documentation for `S`.
/// We also had to derive `Debug` to avoid a warning.
#[derive(Debug)]
pub(crate) struct S;

/// Here is the required documentation
/// for the main function.
pub(crate) fn main() {
    let s = S;
    println!("{s:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
