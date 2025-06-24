#![allow(dead_code)]
// ANCHOR: example
#![allow(clippy::useless_vec)]
#![allow(clippy::manual_contains)]

/// This example demonstrates the difference between `iter()`, `iter_mut()` and
/// `into_iter()`.
fn main() {
    let vec1 = vec![1, 2, 3];

    // `into_iter()` for `Vec<i32>` yields `i32`.
    // It moves the elements out of the vector,
    // so `vec1` CANNOT be used again.
    println!("`2` exists in `vec1`: {}", vec1.into_iter().any(|x| x == 2));
    // ERROR: println!("{:?}", vec1);

    // `any` above tests if any element of the iterator matches a predicate.

    // Very commonly, you'll use a `for` loop to process each element.
    // A `for` loop operating on a collection calls `into_iter()` implicitly.
    let vec2 = vec![10, 20, 30];
    for x in vec2 {
        // Equivalent to vec2.into_iter()
        print!("{:?} ", x);
    }
    println!();

    // `iter()` for `Vec<i32>` yields `&i32`.
    let vec3 = vec![42, 43, 44];
    println!("`42` exists in `vec3`: {}", vec3.iter().any(|&x| x == 42));

    // `iter()` only borrows `vec1` and its elements,
    // so they can be used again.
    for x in &vec3 {
        // Calls `iter()` under the covers.
        print!("{:?} ", x);
    }
    println!();

    // You can modify values of a collection through the mutable reference
    // provided by `iter_mut()`.
    // Note that the vector has to be `mut`.
    let mut vec4 = vec![4, 5, 6];
    for x in vec4.iter_mut() {
        *x *= 2;
    }
    println!("The modified vector is: {:?}", vec4);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
