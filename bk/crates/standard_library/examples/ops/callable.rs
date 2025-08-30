#![allow(dead_code)]
// ANCHOR: example
struct Adder {
    offset: i32,
}

use std::ops::FnOnce;

impl FnOnce<(i32,)> for Adder {
    type Output = i32;

    extern "rust-call" fn call_once(self, args: (i32,)) -> i32 {
        // args is a 1-tuple: `(i32,)`.
        self.offset + args.0
    }
}
use std::ops::FnMut;

impl FnMut<(i32,)> for Adder {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> i32 {
        self.offset + args.0
    }
}
use std::ops::Fn;

impl Fn<(i32,)> for Adder {
    extern "rust-call" fn call(&self, args: (i32,)) -> i32 {
        self.offset + args.0
    }
}

fn main() {
    let add_five = Adder { offset: 5 };

    // Call the type like a function:
    assert_eq!(add_five(3), 8);
    println!("5 + 3 = {}", add_five(3)); // prints "5 + 3 = 8".
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
