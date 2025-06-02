// ANCHOR: example
#![allow(dead_code)]
//! Example of lifetime parameters in various items.

// Function with a lifetime parameter:
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Type alias with a lifetime parameter:
type StrRef<'a> = &'a str;

// Struct with lifetime and type parameters:
struct RefHolder<'a, T> {
    reference: &'a T,
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

// Generic implementation of a trait for a struct.
// Note how the lifetime parameter appears after `impl` and in the name of the
// trait and struct.
impl<'a, T: std::fmt::Debug> Describable<'a> for RefHolder<'a, T> {
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
