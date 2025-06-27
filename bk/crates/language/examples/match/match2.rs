#![allow(dead_code)]
#![allow(clippy::match_single_binding)]
// ANCHOR: example
// Define a struct named `Point` with three fields: x, y, and z,
// all of type `i32`.
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point { x: 0, y: 0, z: 1 };

    // Use `match` to perform pattern matching against a `struct`:
    match p {
        // Match against a single `Point` instance:
        Point { x: 0, y: 0, z: 0 } => println!("p is the origin."),

        // Match against all Points with x = 0.
        // The values of `y` and `z` are bound to variables `b` and `c`.
        Point { x: 0, y: b, z: c } => {
            println!("x: 0, y: {b}, z: {c}")
        } // Note the use of a block after `=>`.

        // The following pattern matches any `Point` struct not caught earlier.
        // It binds the value of the x field to a variable named `x`.
        // The `..` syntax means "ignore the other fields".
        // In this case, `y` and `z` are ignored.
        Point { x, .. } => println!("x is {x}"),
    }
    // Note that the expressions after `=>` return `()`.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
