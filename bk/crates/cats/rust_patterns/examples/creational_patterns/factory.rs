#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A factory is quite useful when we need to create objects of different types
//! that share a common interface, without needing to know the exact type at
//! compile time.

// Define a common trait for all shapes.
// This defines the common interface that all concrete shape types will
// implement.
/// A shape that can be drawn.
trait Shape {
    /// Draws the shape.
    fn draw(&self);
}

// A concrete implementation of the Shape trait.
/// A circle.
struct Circle {
    /// The radius of the circle.
    radius: f64,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

// Another concrete implementation.
/// A square.
struct Square {
    /// The length of a side of the square.
    side: f64,
}

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a square with side: {}", self.side);
    }
}

// A enum that represents the types of shapes we can create.
/// The kinds of shapes that the factory can create.
enum ShapeType {
    /// A circle.
    Circle,
    /// A square.
    Square,
}

// The factory.
/// A factory of shapes.
struct ShapeFactory;

impl ShapeFactory {
    // The logic for creating different shape objects is centralized.
    /// Creates a shape of the given type and size: the radius of a circle or
    /// the side of a square.
    fn create_shape(shape_type: ShapeType, size: f64) -> Box<dyn Shape> {
        match shape_type {
            ShapeType::Circle => Box::new(Circle { radius: size }),
            ShapeType::Square => Box::new(Square { side: size }),
        }
    }
}

/// Creates shapes with the factory and draws them.
fn main() {
    // Using the factory to create different shapes.
    let circle = ShapeFactory::create_shape(ShapeType::Circle, 5.0);
    let square = ShapeFactory::create_shape(ShapeType::Square, 10.0);

    // We use the ShapeFactory to create instances of Circle and Square without
    // directly knowing their concrete types. We only interact with them
    // through the Shape trait.
    circle.draw();
    square.draw();
}

// ANCHOR_END: example

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
