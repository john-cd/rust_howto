// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates vector operations using the `nalgebra` crate:
//! creating vectors, computing dot and cross products, norm, and normalization.

use nalgebra::Vector3;

/// This example demonstrates vector operations using the `nalgebra` crate:
/// creating vectors, computing dot and cross products, norm, and normalization.
fn main() {
    // Create two 3D vectors.
    let v1 = Vector3::new(1.0_f64, 2.0, 3.0);
    let v2 = Vector3::new(4.0_f64, 5.0, 6.0);

    println!("v1 = {v1}");
    println!("v2 = {v2}");

    // Dot product.
    let dot = v1.dot(&v2);
    println!("v1 · v2 = {dot}");

    // Cross product (3D only).
    let cross = v1.cross(&v2);
    println!("v1 × v2 = {cross}");

    // Euclidean norm (magnitude).
    let norm = v1.norm();
    println!("||v1|| = {norm}");

    // Normalize v1 to a unit vector.
    let unit = v1.normalize();
    println!("v1 normalized = {unit}");
    println!("||v1 normalized|| = {}", unit.norm());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the nalgebra vectors example through its entry point.
    #[test]
    fn test_nalgebra_vectors_example_entry_point() {
        main();
    }

    /// Dot, cross and norm of the example vectors; normalizing yields a unit
    /// vector, but a zero vector cannot be normalized.
    #[test]
    fn vector_operations() {
        let v1 = Vector3::new(1.0_f64, 2.0, 3.0);
        let v2 = Vector3::new(4.0_f64, 5.0, 6.0);
        assert!((v1.dot(&v2) - 32.0).abs() < 1e-12);
        assert_eq!(v1.cross(&v2), Vector3::new(-3.0, 6.0, -3.0));
        assert!((v1.norm() - 14.0_f64.sqrt()).abs() < 1e-12);
        assert!((v1.normalize().norm() - 1.0).abs() < 1e-12);
        // The cross product is orthogonal to both inputs.
        assert!(v1.cross(&v2).dot(&v1).abs() < 1e-12);
        assert!(Vector3::<f64>::zeros().try_normalize(1e-12).is_none());
    }
}
