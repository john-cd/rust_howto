#![allow(dead_code)]
// ANCHOR: example
// This attribute tells the compiler to automatically generate
// an implementation of the `std::fmt::Debug` trait for the `Point` struct.
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };

    // Because we derived `Debug`, we can print the struct using the `{:?}`
    // format specifier. Without `#[derive(Debug)]`, this line would cause a
    // compile-time error.
    println!("The point is: {p1:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
