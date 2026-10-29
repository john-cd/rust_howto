// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to invert a 3x3 matrix using the `nalgebra`
//! crate.

use nalgebra::Matrix3;

/// This example demonstrates how to invert a 3x3 matrix using the `nalgebra`
/// crate.
fn main() {
    // Create a 3x3 matrix.
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    // Print the matrix.
    println!("m1 = {m1}");
    // Try to invert the matrix.
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {inv}");
        }
        None => {
            println!("m1 is not invertible!");
        }
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the invert matrix example through its entry point.
    #[test]
    fn test_invert_matrix_example_entry_point() {
        main();
    }

    /// The inverse multiplies back to the identity; singular matrices have no
    /// inverse.
    #[test]
    fn inverts_regular_matrices_only() {
        let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
        let inv = m1.try_inverse().expect("m1 is invertible");
        assert!((m1 * inv - Matrix3::identity()).norm() < 1e-12);

        let singular =
            Matrix3::new(1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 0.0, 1.0, 1.0);
        assert!(singular.try_inverse().is_none());
    }
}
