#![allow(dead_code)]
// ANCHOR: example
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::array;

/// Calculates the L1 norm (Manhattan norm) of a vector.
///
/// The L1 norm is the sum of the absolute values of the vector's elements.
fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

/// Calculates the L2 norm (Euclidean norm) of a vector.
///
/// The L2 norm is the square root of the sum of the squares of the vector's
/// elements.
fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

/// Normalizes a vector to have a unit L2 norm.
fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e / norm);
    x
}

fn main() {
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
