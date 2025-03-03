#![allow(dead_code)]
#![allow(clippy::match_single_binding)]
// ANCHOR: example

// struct pattern matching
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        // Ignoring all fields of a Point except for x by using ..
        Point { x, .. } => println!("x is {}", x),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
