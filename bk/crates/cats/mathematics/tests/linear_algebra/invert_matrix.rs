// ANCHOR: example
use nalgebra::Matrix3;

/// This example demonstrates how to invert a 3x3 matrix using the `nalgebra`
/// crate.
fn main() {
    // Create a 3x3 matrix.
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    // Print the matrix.
    println!("m1 = {}", m1);
    // Try to invert the matrix.
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
