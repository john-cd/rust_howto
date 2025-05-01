#![allow(dead_code)]
#![allow(clippy::match_single_binding)]
// ANCHOR: example
// Define a struct named `Point` with three fields: x, y, and z, all of type
// i32. This struct will be used for pattern matching in the match expression.
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// The main function where the example code resides.
fn main() {
    // Create an instance of the `Point` struct.
    let origin = Point { x: 0, y: 0, z: 0 };

    // Use a match expression to perform pattern matching on the origin `Point`.
    match origin {
        // This pattern matches any `Point` struct.
        // It binds the value of the x field to a variable named x.
        // The .. syntax means "ignore the other fields".
        // In this case, y and z are ignored.
        Point { x, .. } => println!("x is {}", x),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
