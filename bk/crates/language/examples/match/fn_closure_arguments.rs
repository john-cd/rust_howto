#![allow(dead_code)]
// ANCHOR: example
// Destructure a tuple passed as an argument:
fn print_coordinates((x, y): (i32, i32)) {
    println!("x: {x}, y: {y}");
}

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

#[test]
fn test() {
    main();
}
