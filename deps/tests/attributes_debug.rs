#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// or simply #![allow(unused)]
#![allow(dead_code)]
#![allow(missing_docs)]

use std::thread;

pub struct S;

#[must_use]
fn required() -> u32 {
    42
}

fn dead_code() {}

#[test]
fn test() {
    let x = 1;
    let mut m = 2;
}
