// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the attributes derive example through its entry point.
    #[test]
    fn test_attributes_derive_example_entry_point() {
        main();
    }

    /// The derived `Debug` implementation lists the fields.
    #[test]
    fn test_point_debug_output() {
        let p = Point { x: 10, y: 20 };
        assert_eq!(format!("{p:?}"), "Point { x: 10, y: 20 }");
        assert_eq!(format!("{p:#?}"), "Point {\n    x: 10,\n    y: 20,\n}");
    }
}
