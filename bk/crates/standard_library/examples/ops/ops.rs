// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Overloading the `+` operator with `Add`.
//!
//! Operator overloading means implementing the traits in `std::ops`, such as
//! `Add` for `+`. It suits types with a mathematical meaning (vectors, money,
//! durations); avoid it where the operator would surprise readers.

use std::ops::Add;

// The struct we want to overload the '+' operator for.
// We derive a few convenience traits, incl. `Debug` to allow for easy printing
// with `{:?}`.
/// A point in two dimensions.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    /// The x coordinate.
    x: i32,
    /// The y coordinate.
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Adds two points.
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops_example_entry_point() {
        main();
    }

    #[test]
    fn test_add() {
        let p = Point { x: 1, y: 5 } + Point { x: 2, y: 3 };
        assert_eq!(p, Point { x: 3, y: 8 });
    }

    #[test]
    fn test_add_with_negatives_and_zero() {
        let zero = Point { x: 0, y: 0 };
        let p = Point { x: -1, y: 4 };
        assert_eq!(p + zero, p);
        assert_eq!(p + Point { x: 1, y: -4 }, zero);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_add_overflow_panics_in_debug_builds() {
        // Overflow checks are enabled in the test profile.
        let _ = Point { x: i32::MAX, y: 0 } + Point { x: 1, y: 0 };
    }
}
