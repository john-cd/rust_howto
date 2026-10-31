// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Types that implement `Copy` are copied instead of moved.
//!
//! Small, simple types such as integers implement `Copy`: assigning them copies
//! the bits and the original stays usable. Types that own resources, such as
//! `String`, are moved instead.

/// Copies integers and a user-defined `Copy` type.
fn main() {
    // Integers implement the `Copy` trait, so they are bit-wise copied instead
    // of "moved".
    let x = 5; // x is an integer.
    let y = x; // y is a copy of x.

    // Both x and y remain valid:
    println!("x = {x}, y = {y}");

    // It is possible to make a custom type `Copy` by using the `derive`
    // attribute:
    /// Tuple struct whose `i32` value is copied when assigned.
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
    println!("a = {a:?}, b = {b:?}");
    println!("a.0 = {}, b.0 = {}", a.0, b.0);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the copy example through its entry point.
    #[test]
    fn test_copy_example_entry_point() {
        main();
    }

    // Output-only example: `main` only prints the copies.
}
