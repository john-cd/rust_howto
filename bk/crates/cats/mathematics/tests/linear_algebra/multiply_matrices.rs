// ANCHOR: example
use ndarray::arr2;

/// This example demonstrates matrix multiplication using the `ndarray` crate.
fn main() {
    // Define the first matrix 'a' as a 2x3 array.
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    // Define the second matrix 'b' as a 3x2 array.
    // Note that the number of columns in 'a' must match the number of rows in
    // 'b' for matrix multiplication.
    let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("{}", a.dot(&b));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
