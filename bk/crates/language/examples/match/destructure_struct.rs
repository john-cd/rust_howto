#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Destructuring a struct into its fields.

/// A point in two dimensions.
#[derive(Clone, Copy)]
struct Point {
    /// The x coordinate.
    x: i32,
    /// The y coordinate.
    y: i32,
}

/// Destructures a point in `let` and in `match`.
fn main() {
    let p = Point { x: 0, y: 7 };

    // Destructure `p` into variables `a` and `b` in a `let` statement:
    let Point { x: a, y: b } = p;
    println!("a: {a}, b: {b}");

    // Destructure `p` into `x` and `y` variables (shortcut):
    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    // Destructure `p` in a `match` arm:
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the destructure struct example through its entry point.
    #[test]
    fn test_destructure_struct_example_entry_point() {
        main();
    }
}
