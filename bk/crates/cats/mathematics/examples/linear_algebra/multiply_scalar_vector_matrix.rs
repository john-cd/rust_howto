// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to multiply a scalar by a vector and then
//! multiply a matrix by the resulting vector.

use ndarray::Array1;
use ndarray::arr1;
use ndarray::arr2;

/// Multiplies a vector by a scalar, then multiplies a matrix by the result.
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the multiply scalar vector matrix example through its entry point.
    #[test]
    fn test_multiply_scalar_vector_matrix_example_entry_point() {
        main();
    }

    /// Scaling then multiplying gives the expected vector.
    #[test]
    fn scales_and_multiplies() {
        let vector = arr1(&[1, 2, 3]);
        let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);
        let scaled: Array1<_> = 4 * vector;
        assert_eq!(scaled, arr1(&[4, 8, 12]));
        assert_eq!(matrix.dot(&scaled), arr1(&[4 * 32, 4 * 50]));
        let negated: Array1<i32> = -1 * scaled;
        assert_eq!(negated, arr1(&[-4, -8, -12]));
    }
}
