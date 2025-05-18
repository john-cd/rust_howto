// ANCHOR: example
//! Example of using `Box` as a smart pointer for trait objects.
//!
//! `Box` can be used to store an owned trait object on the heap.

// Let's define an example trait and implement it on two structs.
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

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

#[test]
fn test() {
    main();
}
