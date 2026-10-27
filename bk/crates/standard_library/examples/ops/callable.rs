#![allow(dead_code)]

// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Making a struct callable by implementing the `Fn` traits.

/// A callable that adds an offset.
struct Adder {
    /// The number to add.
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

/// Calls an `Adder` like a function.
fn main() {
    let add_five = Adder { offset: 5 };

    // Call the type like a function:
    assert_eq!(add_five(3), 8);
    println!("5 + 3 = {}", add_five(3)); // prints "5 + 3 = 8".
}
// ANCHOR_END: example

#[cfg(test)]
mod tests { use super::*;

    /// Runs the callable example through its entry point.
    #[test]
    fn test_callable_example_entry_point() {
        main();
    }
}
