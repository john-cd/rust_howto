#![allow(dead_code)]
// ANCHOR: example
use std::fmt::Display;

/// A generic struct `Point` that can hold two values of the same type `T`.
struct Point<T> {
    x: T,
    y: T,
}

/// Implementation block for `Point<T>` where `T` can be any type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// Implementation block for Point<f32> only.
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
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
