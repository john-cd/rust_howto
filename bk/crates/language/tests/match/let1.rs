// ANCHOR: example
fn main() {
    // Destructure the tuple into 2 variables:
    let t: (i32, i32) = (1, 2);
    let (x, y) = t;
    // `x` and `y` are `i32`.
    println!("x: {}, y: {}", x, y);

    // The destructuring must always succeed (unless there is an `else` clause -
    // see example below.)
    // let (3, y) = t; // ERROR: refutable pattern in local binding.

    // Destructuring works with structs:
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    let Point { x, y } = origin;
    // `x` and `y` are `i32`.
    println!("x: {}, y: {}", x, y);

    // With arrays:
    let arr: [i32; 3] = [1, 2, 3];
    let [first, _, third] = arr; // Here, we ignore the second value.
    // `first` and `third` are `i32`.
    println!("first: {}, third: {}", first, third);

    let arr = [0, 1, 2, 3, 4];
    let [start, middle @ .., end] = arr; // Bind the middle of the array.
    println!("start: {}, middle: {:?}, end: {}", start, middle, end);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
