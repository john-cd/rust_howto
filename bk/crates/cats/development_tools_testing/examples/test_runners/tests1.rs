//! Writing tests that pass, fail, return a `Result` or are ignored.

#![allow(dead_code)]
#![allow(clippy::eq_op)]

/// Runs the example.
pub fn run() {
    main();
}

// TODO Reviewed ??-??-2026

// ANCHOR: example
// This example demonstrates how to write unit tests.

/// Represents a rectangle with a width and height.
struct Rectangle {
    /// Width of the rectangle.
    width: u32,
    /// Height of the rectangle.
    height: u32,
}

impl Rectangle {
    /// Checks if this rectangle can hold another rectangle.
    fn can_hold(&self, another: &Self) -> bool {
        self.width > another.width && self.height > another.height
    }
}

/// Checks whether a larger rectangle can hold a smaller one.
fn main() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    println!(
        "Can the larger rectangle hold the smaller one? {}",
        larger.can_hold(&smaller)
    );
}

/// This module contains unit tests for the `Rectangle` struct.
/// You will typically write the unit tests in the same file than the main code.
/// The `cfg` attribute ensures that the tests are conditionally compiled only
/// when running `cargo test`.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a larger rectangle can contain a smaller one.
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
        // You may also use `assert_eq!(result, some_const);`
        // or `assert_ne!(...)`.
    }

    /// Verifies that the expected panic makes this test pass.
    #[should_panic]
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    /// Verifies that returning `Ok` marks a result-returning test as passing.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // The test passes if it returns `Ok`.
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /// Long-running test that is skipped during the normal test run.
    #[ignore = "This test takes an hour to run. Only run it manually when needed"]
    #[test]
    fn expensive_test() {
        // Long-running code goes here.
    }
}
// ANCHOR_END: example
