// ANCHOR: example
use ndarray::arr2;

/// This example demonstrates how to add two matrices using the `ndarray` crate.
fn main() {
    // Define the first matrix.
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    // Define the second matrix.
    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
