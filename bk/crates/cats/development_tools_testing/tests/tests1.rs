#![allow(dead_code)]
#![allow(clippy::eq_op)]
// ANCHOR: example
//! This example demonstrates how to write unit tests.

/// Represents a rectangle with a width and height.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Checks if this rectangle can hold another rectangle.
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}

/// This module contains unit tests for the `Rectangle` struct.
/// You will typically write the unit tests in the same file than the main code.
/// The `cfg` attribute ensures that the tests are conditionally compiled only
/// when running `cargo test`.
#[cfg(test)]
mod tests {
    // The following provides access to all objects in the parent module,
    // which contains the main code.
    use super::*;

    /// Test functions must be free, monomorphic functions that take no
    /// arguments, and commonly return `()` or `Result<T, E>` where T:
    /// Termination, E: Debug
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        // You may also use: assert_eq!(result, some_const);
        // or assert_ne!(...)
    }

    /// This test passes if the code inside the function panics;
    /// It fails if the code inside the function doesn't panic.
    #[should_panic]
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    /// Example of test returning `Result`.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // The test passes if it returns `Ok`.
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /// You may use the `ignore` attribute to bypass a long test during normal
    /// testing.
    #[ignore = "This test takes an hour to run. Only run it manually when needed"]
    #[test]
    fn expensive_test() {
        // Long-running code goes here.
    }
}
// ANCHOR_END: example
