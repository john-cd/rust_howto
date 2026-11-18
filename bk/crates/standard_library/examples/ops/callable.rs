// TODO Reviewed ??-??-2026.

//! Making a struct callable by implementing the `Fn` traits.
//!
//! A closure is an anonymous function that can capture variables. The `Fn`,
//! `FnMut` and `FnOnce` traits say how it uses what it captured (by reference,
//! by mutable reference, or by consuming it). Functions that take closures are
//! generic over them.

// ANCHOR: example
#![feature(unboxed_closures, fn_traits)]

/// A callable that adds an offset.
struct Adder {
    /// The number to add.
    offset: i32,
}

use std::ops::FnOnce;

impl FnOnce<(i32,)> for Adder {
    /// Alias for `i32` used to represent output.
    type Output = i32;

    /// Consumes the callable and adds its offset to the tuple-wrapped argument.
    extern "rust-call" fn call_once(self, args: (i32,)) -> i32 {
        // args is a 1-tuple: `(i32,)`.
        self.offset + args.0
    }
}

use std::ops::FnMut;

impl FnMut<(i32,)> for Adder {
    /// Adds the offset through a mutable receiver without changing stored state.
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> i32 {
        self.offset + args.0
    }
}

use std::ops::Fn;

impl Fn<(i32,)> for Adder {
    /// Adds the offset through a shared receiver so calls can be repeated.
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

/// Checks callable syntax, repeated calls, and the three closure trait bounds.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the `callable` example runs.
    #[test]
    fn test_callable_example_entry_point() {
        main();
    }

    /// Verifies adder is callable like a function.
    #[test]
    fn test_adder_is_callable_like_a_function() {
        let add_five = Adder { offset: 5 };
        assert_eq!(add_five(3), 8);
        assert_eq!(add_five(-5), 0);
        // `Fn` can be called repeatedly.
        assert_eq!(add_five(3), add_five(3));
    }

    /// Verifies adder as fn trait bounds.
    #[test]
    fn test_adder_as_fn_trait_bounds() {
        /// Calls a shared callable with 1 to exercise the Fn bound.
        fn call_fn(f: impl Fn(i32) -> i32) -> i32 {
            f(1)
        }
        /// Calls a mutable callable with 1 to exercise the FnMut bound.
        fn call_fn_mut(mut f: impl FnMut(i32) -> i32) -> i32 {
            f(1)
        }
        /// Consumes a callable with argument 1 to exercise the FnOnce bound.
        fn call_fn_once(f: impl FnOnce(i32) -> i32) -> i32 {
            f(1)
        }
        assert_eq!(call_fn(Adder { offset: 2 }), 3);
        assert_eq!(call_fn_mut(Adder { offset: 2 }), 3);
        assert_eq!(call_fn_once(Adder { offset: 2 }), 3);
    }
}
