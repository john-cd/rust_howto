#![allow(dead_code)]
// ANCHOR: example
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
    #[test]
    fn test() {
        main();
    }
}
