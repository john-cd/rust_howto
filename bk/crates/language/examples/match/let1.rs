#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let t: (i32, i32) = (1, 2);
    // Destructure the tuple into 2 variables:
    let (x, y) = t;
    // `x` and `y` are `i32`.
    println!("x: {x}, y: {y}");

    // The destructuring must always succeed (unless there is an `else` clause -
    // see example below.)
    // let (3, y) = t; // ERROR: refutable pattern in local binding.

    // Destructuring with structs:
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    let Point { x, y } = origin;
    // `x` and `y` are `i32`.
    println!("x: {x}, y: {y}");

    // Destructuring with arrays:
    let arr: [i32; 3] = [1, 2, 3];
    let [first, _, third] = arr; // Here, we ignore the second value.
    // `first` and `third` are `i32`.
    println!("first: {first}, third: {third}");

    // Bind the middle of an array:
    let arr = [0, 1, 2, 3, 4];
    let [start, middle @ .., end] = arr;
    println!("start: {start}, middle: {middle:?}, end: {end}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
