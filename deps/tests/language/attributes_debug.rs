// ANCHOR: example
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// or simply: #![allow(unused)]
#![allow(dead_code)]
#![allow(missing_docs)]

// This import is not used anywhere
use std::thread;

// This struct is public but is not documented
pub struct S;

#[must_use]
fn required() -> u32 {
    42
}

// Nothing calls this function
fn dead_code() {}

fn main() {
    // This variable is not used
    let x = 1;
    // This mutable variable is not used
    let mut m = 2;
    // The return value of this function is not used
    required();
    println!("Done!");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO P1
