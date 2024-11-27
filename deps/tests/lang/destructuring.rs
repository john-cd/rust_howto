// ANCHOR: example
fn main() {
    // destructuring tuples
    let (_x, _y, _z) = (1, 2, 3);

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: _a, y: _b } = p; // a = 0, b = 7
    let Point { x, y } = p; // simpler
    let _ = (x, y);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO P1
