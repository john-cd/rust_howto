#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // Integers implement the `Copy` trait, so they are bit-wise copied instead
    // of "moved".
    let x = 5; // x is an integer.
    let y = x; // y is a copy of x.

    // Both x and y remain valid:
    println!("x = {}, y = {}", x, y);

    // It is possible to make a custom type `Copy` by using the `derive`
    // attribute:
    #[derive(Copy, Clone, Debug)]
    struct S(i32);
    // Notes:
    // - `Clone` is a supertrait of `Copy`, so everything which is
    // `Copy` must also implement `Clone`.
    // - `#[derive(Copy)]` requires that all of the struct's components
    //   implement `Copy`.
    // - You could also implement `Copy` and `Clone` manually with `impl`
    //   blocks.

    let a = S(5); // `a` is a struct that implements `Copy`.
    let b = a; // `b` is a copy of `a`.
    // Both `a` and `b` remain valid:
    println!("a = {:?}, b = {:?}", a, b);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
