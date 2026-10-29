// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Generic functions with trait bounds.

use std::fmt::Debug;
use std::fmt::Display;

/// Generic functions can accept trait bounds.
// Here we define a generic function `print_value` that takes one argument
// `value` of any type `T`. The `T: std::fmt::Debug` part is a trait bound,
// meaning `T` must implement the `Debug` trait, so that it can be printed using
// the `{:?}` format specifier.
/// Prints `value` using its `Debug` representation.
fn print_value<T: Debug>(value: T) {
    println!("The value is: {value:?}"); // `{:?}` can be used, because `value` implements `Debug`.
}

/// Type constraints can be written in a separate `where` clause for clarity.
/// We can also combine multiple trait bounds with `+`.
fn print_value2<T>(value: T)
where
    T: Debug + Display,
{
    println!("Compare {value} and {value:?}");
}

/// All type parameters have an implicit bound of `Sized`.
/// The special syntax `?Sized` can be used to remove this bound if it is not
/// appropriate.
///
/// This function can work with types that may not have a known size at compile
/// time.
fn generic<T: ?Sized + Display>(t: &T) {
    println!("{t}");
}

/// Calls the generic functions with several types.
fn main() {
    // Call the generic function with an integer.
    print_value(10);

    // Call the generic function with a floating-point number.
    print_value(2.14);

    // Call the generic function with a string slice.
    print_value("hello");

    // Call the generic function with a custom struct that implements `Debug`.
    /// Holds a value of any type to demonstrate generic struct implementations.
    #[expect(dead_code, reason = "declared to illustrate it; never used")]
    #[derive(Debug)]
    struct MyStruct {
        /// The value stored by this wrapper.
        value: i32,
    }
    print_value(MyStruct { value: 42 });

    let s = String::from("hello");
    generic(&*s);
    generic(&s);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the generic functions2 example through its entry point.
    #[test]
    fn test_generic_functions2_example_entry_point() {
        main();
    }

    // Output-only example: the generic functions only print their argument.
}
