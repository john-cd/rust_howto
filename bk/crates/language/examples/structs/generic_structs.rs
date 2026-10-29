// TODO Reviewed ??-??-2026.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the snippet declares items to illustrate them, not to call them"
    )
)]
// ANCHOR: example
//! A generic struct and its `impl` blocks.

use std::fmt::Display;

/// A generic struct `Point` that can hold two values of the same type `T`:
struct Point<T> {
    /// The x coordinate.
    x: T,
    /// The y coordinate.
    y: T,
}

/// Generic implementation block for `Point<T>` where `T` can be any type:
impl<T> Point<T> {
    /// Returns a reference to the x coordinate.
    fn x(&self) -> &T {
        &self.x
    }
}

/// Implementation block for `Point<f32>` only:
impl Point<f32> {
    /// A method specific to `Point<f32>`:
    fn distance_from_origin(&self) -> f32 {
        self.x.hypot(self.y)
    }
}

/// You may also use Trait Bounds to conditionally implement methods.
/// The following is only implemented for `Point<T>` when `T` implements both
/// the `Display` and `PartialOrd` traits.
impl<T: Display + PartialOrd> Point<T> {
    /// Prints the larger coordinate.
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// Creates points with several types and calls their methods.
fn main() {
    // Instantiate `Point` with integer coordinates (type `i32`).
    let integer_point = Point { x: 5, y: 10 };

    // Instantiate `Point` with floating-point coordinates (type `f32`).
    let float_point: Point<f32> = Point { x: 1.0, y: 4.0 };

    // Rust can often infer the type, but you could also be explicit:
    // let integer_point: Point<i32> = Point { x: 5, y: 10 };
    // let float_point: Point<f32> = Point { x: 1.0, y: 4.0 };

    println!(
        "Integer point: x = {}, y = {}",
        integer_point.x, integer_point.y
    );
    println!("Float point: x = {}, y = {}", float_point.x, float_point.y);
    println!(
        "Distance from origin: {}",
        float_point.distance_from_origin()
    );

    // This would cause a compile-time error because x and y must be the same
    // type `T`: let wont_compile = Point { x: 5, y: 4.0 };
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the generic structs example through its entry point.
    #[test]
    fn test_generic_structs_example_entry_point() {
        main();
    }

    /// The accessor returns the stored value.
    #[test]
    fn test_x() {
        assert_eq!(*Point { x: 5, y: 10 }.x(), 5);
        assert_eq!(*Point { x: "a", y: "b" }.x(), "a");
    }

    /// The Euclidean distance is only defined for `f32` points.
    #[test]
    fn test_distance_from_origin() {
        let p: Point<f32> = Point { x: 3.0, y: 4.0 };
        assert!((p.distance_from_origin() - 5.0).abs() < f32::EPSILON);
        let origin: Point<f32> = Point { x: 0.0, y: 0.0 };
        assert!(origin.distance_from_origin().abs() < f32::EPSILON);
    }

    /// Comparison works for any `Display + PartialOrd` type.
    #[test]
    fn test_cmp_display() {
        Point { x: 1, y: 2 }.cmp_display();
        Point { x: 2.5, y: 1.5 }.cmp_display();
        Point { x: "b", y: "a" }.cmp_display();
    }
}
