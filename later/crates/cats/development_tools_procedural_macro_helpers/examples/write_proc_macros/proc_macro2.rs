//! A derive macro written with `proc-macro2`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
use development_tools_procedural_macro_helpers::MyDebug;

/// A struct that derives `MyDebug`.
#[derive(MyDebug)]
struct MyStruct;

/// Prints a struct with the derived `Debug`.
fn main() {
    let s = MyStruct;
    println!("{:?}", s);
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[test]
fn test() {
    main();
}
