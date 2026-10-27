#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Destructuring in a `for` loop.

/// Destructures the tuples of a vector in a `for` loop.
fn main() {
    // Create a `Vec` of tuples.
    let points = vec![(0, 1), (2, 3), (4, 5)];

    // Iterate through the `Vec`. Each element is deconstructed in turn into
    // variables `x` and `y`.
    for (x, y) in points {
        println!("x: {x}, y: {y}");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the for1 example through its entry point.
    #[test]
    fn test_for1_example_entry_point() {
        main();
    }
}
