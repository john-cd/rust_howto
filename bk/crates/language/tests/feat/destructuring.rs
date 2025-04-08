// ANCHOR: example
fn main() {
    // Destructuring a tuple.
    // The values in the tuple are assigned to the variables in the same order.
    let (x, y, _) = (1, 2, 3);
    // x is assigned the value 1.
    // y is assigned the value 2.
    // Use `_` to ignore a field you don't care about.

    println!("x: {x}, y: {y}");

    struct Point {
        x: i32,
        y: i32,
    }

    // Create a `struct` instance.
    let p = Point { x: 0, y: 7 };

    // Destructuring a struct.
    // The values in the struct are assigned to the variables based on the field
    // names. a is assigned the value of p.x, which is 0.
    // b is assigned the value of p.y, which is 7.
    let Point { x: a, y: b } = p;
    println!("a: {a}, b: {b}");

    // Here is a simpler way to destructure a struct.
    let Point { x, y } = p;
    // This is equivalent to `let Point { x: x, y: y } = p;`.
    print!("x and y: {:?}", (x, y));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
