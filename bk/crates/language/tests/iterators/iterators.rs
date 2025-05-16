// ANCHOR: example
#![allow(clippy::useless_vec)]
#![allow(clippy::manual_contains)]

/// This example demonstrates the difference between `iter()`, `iter_mut()` and
/// `into_iter()`.
fn main() {
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for `Vec<i32>` yields `&i32`,
    // which we destructured via the `&x` pattern.
    // `any` tests if any element of the iterator matches a predicate.
    println!("`2` exists in `vec1`: {}", vec1.iter().any(|&x| x == 2));
    // `iter()` only borrows `vec1` and its elements,
    // so they can be used again. Note the (very common) use of a `for` loop.
    for x in vec1.iter() {
        print!("{:?}", x);
    }
    println!();
    // You can modify values through the mutable reference provided by
    // `iter_mut`.
    for x in vec1.iter_mut() {
        *x *= 2;
    }
    println!("The modified vector is: {:?}", vec1); // [2, 4, 6]

    // `into_iter()` for vecs yields `i32`. No destructuring is required.
    // `into_iter()` moves `vec2` and its elements,
    // so they CANNOT be used again.
    println!("2 exists in `vec2`: {}", vec2.into_iter().any(|x| x == 2));
    // ERROR: println!("{:?}", vec2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
