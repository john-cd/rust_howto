// ANCHOR: example
//! `allow` attributes suppress specific warnings.
//!
//! These attributes are typically used during eatly development to temporarily
//! ignore warnings that are not relevant at the moment, but should be
//! addressed before releasing the code.
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// Or simply use: `#![allow(unused)]`.
#![allow(dead_code)]
#![allow(missing_docs)]

// This import is not used anywhere:
use std::thread;

// This struct is public but is not documented:
pub struct S;

/// This function is defined but never called.
fn dead_code() {}

/// The return value of this function should be used.
#[must_use]
fn required() -> u32 {
    42
}

fn main() {
    // This variable is defined but not used.
    let x = 1;
    // This mutable variable is defined but not used.
    let mut m = 2;
    // The return value of this function is not used, despite the `#[must_use]`
    // attribute above.
    required();
    println!("Done!");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
