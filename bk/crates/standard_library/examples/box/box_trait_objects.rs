#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Example of using `Box` as a smart pointer for trait objects.
//!
//! `Box` can be used to store an owned trait object on the heap.

// Let's define an example trait and implement it on two structs.
/// A shape with an area.
trait Shape {
    /// Returns the area of the shape.
    fn area(&self) -> f64;
}

/// A circle.
struct Circle {
    /// The radius of the circle.
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// A rectangle.
struct Rectangle {
    /// The width of the rectangle.
    width: f64,
    /// The height of the rectangle.
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// Stores different shapes in a vector of boxed trait objects and prints their
/// areas.
fn main() {
    // Put different concrete types that implement `Shape` into a `Vec<Box<dyn
    // Shape>>`.
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
    ];

    // Dynamic dispatch: the `area` methods of the underlying concrete types
    // (`Circle` then `Rectangle`) are called.
    for shape in shapes {
        println!("Shape area: {:.2}", shape.area());
    }
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
