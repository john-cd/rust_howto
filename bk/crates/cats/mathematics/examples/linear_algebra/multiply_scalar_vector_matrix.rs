#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to multiply a scalar by a vector and then
//! multiply a matrix by the resulting vector.

use ndarray::Array1;
use ndarray::arr1;
use ndarray::arr2;

fn main() {
    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{new_vector}");

    let new_matrix = matrix.dot(&new_vector);
    println!("{new_matrix}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
