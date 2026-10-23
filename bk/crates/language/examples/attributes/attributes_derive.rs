#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Deriving traits with `#[derive]`.

// This attribute tells the compiler to automatically generate
// an implementation of the `std::fmt::Debug` trait for the `Point` struct.
/// A point that derives `Debug`.
#[derive(Debug)]
struct Point {
    /// The x coordinate.
    x: i32,
    /// The y coordinate.
    y: i32,
}

/// Prints a point with the derived `Debug`.
fn main() {
    let p1 = Point { x: 10, y: 20 };

    // Because we derived `Debug`, we can print the struct using the `{:?}`
    // format specifier. Without `#[derive(Debug)]`, this line would cause a
    // compile-time error.
    println!("The point is: {p1:?}");
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
