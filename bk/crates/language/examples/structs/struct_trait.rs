//! Implementing a trait with methods, an associated type and associated
//! constants.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
// First, define a trait (if it's not a pre-existing one):
/// A trait with methods, an associated function, a type and constants.
trait SomeBehavior {
    // Method declaration (signature only, no body):
    /// Does something and returns a description.
    fn do_something(&self) -> String;

    // Method with default implementation:
    /// Performs an action with `value`; has a default implementation.
    fn another_action(&self, value: i32) {
        println!("The default implementation received the value: {value}");
    }

    // Associated function. In this case, it references the associated type
    // defined below. Note the path syntax `Self::<AssociatedType>`.
    /// An associated function that returns the associated type.
    fn assoc_func() -> Self::Output;

    // Associated type:
    /// The type that the associated function returns.
    type Output;

    // Associated constants with and without default:
    /// A constant with a default value.
    const CONSTANT: i32 = 42;

    /// A constant without a default value.
    const ANOTHER_CONST: &str;
}

// Define your struct:
/// A struct that implements [`SomeBehavior`].
struct MyStruct {
    /// A number.
    data: i32,
    /// A name.
    name: String,
}

// Implement the trait for your struct:
impl SomeBehavior for MyStruct {
    // Define the associated type:
    type Output = String;

    // Define the const that did not have a default:
    const ANOTHER_CONST: &str = "Another constant value";
    // Override the const that did have a default:
    const CONSTANT: i32 = 43;

    // Concrete implementation of a trait method for this struct:
    fn do_something(&self) -> String {
        format!("{} is doing something with data: {}", self.name, self.data)
    }

    // Override the default implementation of a method:
    fn another_action(&self, value: i32) {
        println!("{} received value: {}", self.name, value);
    }

    // Implement the associated function:
    fn assoc_func() -> Self::Output {
        "Associated function output".to_owned()
    }
}

/// Uses the trait implementation.
fn main() {
    let m = MyStruct {
        data: 42,
        name: "Alice".to_owned(),
    };
    // Use the `.` operator to call a method on a type that implements it:
    println!("{}", m.do_something());
    m.another_action(10);

    println!("{}", MyStruct::assoc_func());
    println!("{}", MyStruct::CONSTANT);
    println!("{}", MyStruct::ANOTHER_CONST);
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
