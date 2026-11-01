// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to add two matrices using the `ndarray` crate.
//!
//! A matrix is a rectangular grid of numbers. Matrices are used for graphics
//! transformations, machine learning and solving systems of equations. Adding
//! two matrices of the same shape adds element by element.

use ndarray::arr2;

/// This example demonstrates how to add two matrices using the `ndarray` crate.
fn main() {
    // Define the first matrix.
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    // Define the second matrix.
    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{a}");
    println!("+");
    println!("{b}");
    println!("=");
    println!("{sum}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the add matrices example through its entry point.
    #[test]
    fn test_add_matrices_example_entry_point() {
        main();
    }

    /// Element-wise addition gives the expected sums; mismatched shapes panic.
    #[test]
    fn adds_elementwise() {
        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let b = arr2(&[[6, 5, 4], [3, 2, 1]]);
        assert_eq!(&a + &b, arr2(&[[7, 7, 7], [7, 7, 7]]));
        assert_eq!(&b + &a, &a + &b);
    }

    /// Adding matrices of different shapes is a programming error.
    #[test]
    #[should_panic(expected = "incompatible shapes")]
    fn mismatched_shapes_panic() {
        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let b = arr2(&[[1, 2], [3, 4]]);
        let _ = &a + &b;
    }
}
