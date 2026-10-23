#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `Either` enum from the `either`
//! crate.
//!
//! The `Either` enum is a type that represents a value that can be one of two
//! possible types. It is commonly used in functional programming to handle
//! situations where a function might return one of two different types.

use either::Either;

/// Creates and matches `Either` values.
fn simple() {
    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("hello");

    match left {
        Either::Left(value) => println!("Left value: {value}"),
        Either::Right(value) => println!("Right value: {value}"),
    }

    match right {
        Either::Left(value) => println!("Left value: {value}"),
        Either::Right(value) => println!("Right value: {value}"),
    }
}

// Function that can return two different types.
/// Returns the double of `value` on the right, or an error message on the left
/// if `value` is not positive.
fn process_positive_integer(value: i32) -> Either<String, i32> {
    if value > 0 {
        Either::Right(value * 2)
    } else {
        Either::Left(format!("Invalid input: {value}"))
    }
}

/// Runs the `Either` examples and prints their results.
fn main() {
    simple();

    let result1 = process_positive_integer(5);
    let result2 = process_positive_integer(-3);

    match result1 {
        Either::Left(ref err) => println!("Error: {err}"),
        Either::Right(val) => println!("Processed value: {val}"),
    }

    match result2 {
        Either::Left(err) => println!("Error: {err}"),
        Either::Right(val) => println!("Processed value: {val}"),
    }

    // Transforming `Either` values:
    let _mapped_result =
        result1.map_left(|s| s.to_uppercase()).map_right(|n| n + 10);
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
