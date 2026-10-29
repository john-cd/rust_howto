// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates matrix multiplication using the `ndarray` crate.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the multiply matrices example through its entry point.
    #[test]
    fn test_multiply_matrices_example_entry_point() {
        main();
    }

    /// The 2x3 by 3x2 product is the expected 2x2 matrix.
    #[test]
    fn multiplies_matrices() {
        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let b = arr2(&[[6, 3], [5, 2], [4, 1]]);
        assert_eq!(a.dot(&b), arr2(&[[28, 10], [73, 28]]));
        // Multiplying by the identity leaves the matrix unchanged.
        assert_eq!(
            a.mapv(f64::from).dot(&ndarray::Array2::eye(3)),
            a.mapv(f64::from)
        );
    }

    /// Incompatible inner dimensions panic.
    #[test]
    #[should_panic(expected = "ndarray: inputs")]
    fn incompatible_shapes_panic() {
        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let _ = a.dot(&a);
    }
}
