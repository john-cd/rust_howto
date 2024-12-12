// ANCHOR: example
fn main() {
    // Destructuring a tuple
    let (x, y, _) = (1, 2, 3);
    // x, y are now stored individually in two separate `i32` variables
    // Use _ to ignore a field you don't care about.
    println!("x: {x}, y: {y}");

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    // Destructuring a struct - sets a = 0 and b = 7:
    let Point { x: a, y: b } = p;
    println!("a: {a}, b: {b}");

    // Here is a simpler way:
    let Point { x, y } = p;
    // This is equivalent to `let Point { x: x, y: y } = p;``
    print!("x and y: {:?}", (x, y));
    // An underscore can be used as well, if needed.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
