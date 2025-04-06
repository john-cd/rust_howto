// ANCHOR: example
//! This example demonstrates how to compare vectors using the `approx` crate.

use approx::assert_abs_diff_eq;
use ndarray::Array;

fn main() {
    // Create vectors.
    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;

    // Check that the sum of the vectors is correct.
    assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("c = {}", c);

    // Modify the vectors.
    c[0] = 10.;
    d[1] = 10.;

    // Assert approximate equality (using the absolute difference).
    assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
