// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Computing the L1 and L2 norms of a vector, and normalizing it, with
//! `ndarray`.

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

/// Prints the norms of a vector and its normalization.
fn main() {
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the vector norm example through its entry point.
    #[test]
    fn test_vector_norm_example_entry_point() {
        main();
    }

    /// Norms of a known vector; normalized vectors have unit L2 norm.
    #[test]
    fn computes_norms_and_normalizes() {
        let x = array![3., 4.];
        assert!((l2_norm(x.view()) - 5.0).abs() < 1e-12);
        assert!((l1_norm(array![1., -2., 3.].view()) - 6.0).abs() < 1e-12);
        let n = normalize(x);
        assert!((l2_norm(n.view()) - 1.0).abs() < 1e-12);
        assert!((n[0] - 0.6).abs() < 1e-12);
        assert_eq!(l1_norm(Array1::<f64>::zeros(0).view()), 0.0);
    }

    /// Normalizing a zero vector yields NaN, not a panic.
    #[test]
    fn normalizing_zero_vector_gives_nan() {
        let n = normalize(array![0., 0.]);
        assert!(n.iter().all(|e| e.is_nan()));
    }
}
