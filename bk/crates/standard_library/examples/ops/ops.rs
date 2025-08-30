#![allow(dead_code)]
// ANCHOR: example
use std::ops::Add;

// The struct we want to overload the '+' operator for.
// We derive a few convenience traits, incl. `Debug` to allow for easy printing with `{:?}`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// Implementation of the `Add` trait for our `Point` struct.
impl Add for Point {
    // `type Output` specifies what type the `+` operator will return.
    type Output = Self;

    // This is the method that the `+` operator will call.
    // `self` is the left-hand side (e.g., `p1` in `p1 + p2`).
    // `rhs` (right-hand side) is the other operand (e.g., `p2`).
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 5 };
    let p2 = Point { x: 2, y: 3 };

    // Here we use the overloaded '+' operator:
    let p3 = p1 + p2;
    assert_eq!(p3, Point { x: 3, y: 8 });

    println!("{p1:?} + {p2:?} = {p3:?}");
    // Expected output:
    // Point { x: 1, y: 5 } + Point { x: 2, y: 3 } = Point { x: 3, y: 8 }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
