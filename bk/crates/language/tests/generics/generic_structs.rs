#![allow(dead_code)]
// ANCHOR: example
use std::fmt::Display;

/// A generic struct `Point` that can hold two values of the same type `T`:
struct Point<T> {
    x: T,
    y: T,
}

/// Implementation block for `Point<T>` where `T` can be any type:
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// Implementation block for `Point<f32>` only:
impl Point<f32> {
    /// A method specific to `Point<f32>`:
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// You may also use Trait Bounds to conditionally implement methods.
/// The following is only implemented for `Point<T>` when `T` implements both
/// the `Display` and `PartialOrd` traits.
impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    // Instantiate `Point` with integer coordinates (type `i32`).
    let integer_point = Point { x: 5, y: 10 };

    // Instantiate `Point` with floating-point coordinates (type `f64`).
    let float_point = Point { x: 1.0, y: 4.0 };

    // Rust can often infer the type, but you could also be explicit:
    // let integer_point: Point<i32> = Point { x: 5, y: 10 };
    // let float_point: Point<f64> = Point { x: 1.0, y: 4.0 };

    println!(
        "Integer point: x = {}, y = {}",
        integer_point.x, integer_point.y
    );
    println!("Float point: x = {}, y = {}", float_point.x, float_point.y);

    // This would cause a compile-time error because x and y must be the same
    // type `T`: let wont_compile = Point { x: 5, y: 4.0 };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
