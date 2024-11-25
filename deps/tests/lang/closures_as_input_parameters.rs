#![allow(dead_code)]
// ANCHOR: example

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    // The closure takes no input and returns nothing.
    // could also be `Fn` or `FnMut`.
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32
where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    apply(|| println!("Applied"));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
