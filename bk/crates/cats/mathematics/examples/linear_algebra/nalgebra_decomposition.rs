#![allow(dead_code)]
// ANCHOR: example
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

#[test]
fn test() {
    main();
}
