// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Example of lifetime parameters in various items.

// Function with a lifetime parameter `'a`.
// The parameter is used by the function parameter declaration.
// In this case, `s1`, `s2`, and the return value all share the same lifetime.
/// Returns the longest of two string slices, which share the same lifetime.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Type alias with a lifetime parameter:
/// A string slice with a lifetime parameter.
type StrRef<'a> = &'a str;

// Struct with lifetime and type parameters:
/// A struct that holds a reference.
struct RefHolder<'a, T> {
    /// The reference held.
    reference: &'a T,
}

// Implementation for the struct, with lifetime and type parameters.
// Note how the lifetime parameter appears directly after `impl`.
impl<'a, T> RefHolder<'a, T> {
    /// Creates a holder for `reference`.
    fn new(reference: &'a T) -> Self {
        Self { reference }
    }
}

// Enumeration with lifetime parameter:
/// An enum whose variants hold references.
enum Either<'a, T, U> {
    /// A reference to a value of the first type.
    Left(&'a T),
    /// A reference to a value of the second type.
    Right(&'a U),
}

// Union with lifetime parameter (unsafe, rarer):
/// A union of references.
union MyUnion<'a> {
    /// A reference to an integer.
    int_ref: &'a i32,
    /// A reference to a float.
    float_ref: &'a f32,
}

// Trait with a lifetime parameter:
/// A trait with a lifetime parameter.
trait Describable<'a> {
    /// Returns a description that lives for `'a`.
    fn describe(&self) -> &'a str;
}

// Implementation of a generic trait, with a lifetime parameter:
impl<'a> Describable<'a> for String {
    fn describe(&self) -> &'a str {
        "This is a string."
    }
}

use std::fmt::Display;

// Implementation of a trait for a generic type.
// Note the trait bound.
impl<T: Display> Display for RefHolder<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefHolder: {}", self.reference)
    }
}

// Implementation of a generic trait for a generic type:
impl<'a, T> Describable<'a> for RefHolder<'a, T> {
    fn describe(&self) -> &'a str {
        "This is a reference holder."
    }
}

/// Uses lifetime parameters in functions, structs and traits.
fn main() {
    let s1 = "Hello";
    let s2 = "World!";
    println!("Longest: {}", longest(s1, s2));

    let num = 42;
    let holder = RefHolder { reference: &num };
    println!("{}", holder.describe());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the lifetime parameters example through its entry point.
    #[test]
    fn test_lifetime_parameters_example_entry_point() {
        main();
    }
}
