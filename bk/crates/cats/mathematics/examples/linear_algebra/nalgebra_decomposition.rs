// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates matrix decompositions using the `nalgebra` crate.
//!
//! A decomposition splits a matrix into simpler ones (such as LU, QR or SVD).
//! It is the practical way to solve equations, fit curves and compute inverses
//! with good numerical accuracy.

use nalgebra::Matrix3;

/// This example demonstrates matrix decompositions using the `nalgebra` crate.
///
/// Singular Value Decomposition (SVD) factorizes a matrix M into U * Σ * Vᵀ,
/// where U and V are orthogonal matrices and Σ is a diagonal matrix of
/// singular values. SVD has applications in dimensionality reduction,
/// pseudoinverse computation, and solving least-squares problems.
fn main() {
    // Create a 3x3 matrix.
    let m = Matrix3::new(
        3.0_f64, 1.0, 1.0, //
        1.0, 3.0, 1.0, //
        1.0, 1.0, 3.0,
    );
    println!("Matrix M:\n{m}");

    // Compute the SVD: M = U * Σ * Vᵀ.
    // `compute_u` and `compute_v` control whether U and V are computed.
    let svd = m.svd(true, true);
    let singular_values = svd.singular_values;
    println!("Singular values: {singular_values}");

    if let (Some(u), Some(v_t)) = (svd.u, svd.v_t) {
        println!("U matrix:\n{u}");
        println!("Vᵀ matrix:\n{v_t}");

        // Reconstruct the original matrix from U, Σ, and Vᵀ.
        let sigma = Matrix3::from_diagonal(&singular_values);
        let reconstructed = u * sigma * v_t;
        println!("Reconstructed M ≈ U * Σ * Vᵀ:\n{reconstructed}");
    }

    // Compute the determinant and trace as additional matrix properties.
    println!("Determinant of M: {}", m.determinant());
    println!("Trace of M: {}", m.trace());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the nalgebra decomposition example through its entry point.
    #[test]
    fn test_nalgebra_decomposition_example_entry_point() {
        main();
    }

    /// `U * Sigma * V^T` reconstructs the matrix; the singular values are
    /// sorted and the determinant/trace are known.
    #[test]
    fn svd_reconstructs_matrix() {
        let m = Matrix3::new(3.0_f64, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0);
        let svd = m.svd(true, true);
        let (u, v_t) =
            (svd.u.expect("u requested"), svd.v_t.expect("v_t requested"));
        let reconstructed =
            u * Matrix3::from_diagonal(&svd.singular_values) * v_t;
        assert!((reconstructed - m).norm() < 1e-12);
        // Eigenvalues are 5, 2, 2.
        let sv = svd.singular_values;
        assert!((sv.max() - 5.0).abs() < 1e-12);
        assert!((sv.min() - 2.0).abs() < 1e-12);
        assert!((m.determinant() - 20.0).abs() < 1e-12);
        assert!((m.trace() - 9.0).abs() < 1e-12);
    }
}
