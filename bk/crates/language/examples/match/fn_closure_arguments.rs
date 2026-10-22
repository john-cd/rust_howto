#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Destructuring in function and closure arguments.

// Destructure a tuple passed as an argument:
/// Prints the coordinates of a point given as a tuple.
fn print_coordinates((x, y): (i32, i32)) {
    println!("x: {x}, y: {y}");
}

/// Destructures a tuple in the arguments of a function and a closure.
fn main() {
    let point = (10, 20);
    print_coordinates(point);

    // The same works with closure arguments.
    // Here we destructure a mutable reference into `x`, a `i32` variable.
    // Note the second `mut` to make `x` mutable within the closure!
    let add_one = |&mut mut x| -> i32 {
        x += 1;
        x
    };

    let a: &mut i32 = &mut 3;
    let b = add_one(a);
    println!("a: {a}, b: {b}");
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
