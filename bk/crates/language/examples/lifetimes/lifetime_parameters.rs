// ANCHOR: example
#![allow(dead_code)]
//! Example of lifetime parameters in various items.

// Function with a lifetime parameter `'a`.
// The parameter is used by the function parameter declaration.
// In this case, `s1`, `s2`, and the return value all share the same lifetime.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Type alias with a lifetime parameter:
type StrRef<'a> = &'a str;

// Struct with lifetime and type parameters:
struct RefHolder<'a, T> {
    reference: &'a T,
}

// Implementation for the struct, with lifetime and type parameters.
// Note how the lifetime parameter appears directly after `impl`.
impl<'a, T> RefHolder<'a, T> {
    fn new(reference: &'a T) -> Self {
        Self { reference }
    }
}

// Enumeration with lifetime parameter:
enum Either<'a, T, U> {
    Left(&'a T),
    Right(&'a U),
}

// Union with lifetime parameter (unsafe, rarer):
union MyUnion<'a> {
    int_ref: &'a i32,
    float_ref: &'a f32,
}

// Trait with a lifetime parameter:
trait Describable<'a> {
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
impl<'a, T: Display> Display for RefHolder<'a, T> {
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

fn main() {
    let s1 = "Hello";
    let s2 = "World!";
    println!("Longest: {}", longest(s1, s2));

    let num = 42;
    let holder = RefHolder { reference: &num };
    println!("{}", holder.describe());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
