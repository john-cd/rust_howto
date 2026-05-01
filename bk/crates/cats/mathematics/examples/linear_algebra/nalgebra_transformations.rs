#![allow(dead_code)]
// ANCHOR: example
use std::f64::consts::PI;

use nalgebra::Isometry3;
use nalgebra::Point3;
use nalgebra::Rotation3;
use nalgebra::Translation3;
use nalgebra::Unit;
use nalgebra::Vector3;

/// This example demonstrates transformations in 3D using the `nalgebra` crate:
/// rotations, translations, and composing them into isometries.
fn main() {
    // Create a 90° rotation around the Z axis.
    let axis = Unit::new_normalize(Vector3::z());
    let rotation = Rotation3::from_axis_angle(&axis, PI / 2.0);
    println!("Rotation (90° around Z):\n{rotation}");

    // Apply the rotation to a point.
    let point = Point3::new(1.0_f64, 0.0, 0.0);
    let rotated = rotation * point;
    println!("Rotating {point} by 90° around Z gives: {rotated}");

    // Create a translation.
    let translation = Translation3::new(0.0_f64, 1.0, 2.0);

    // Compose the rotation and translation into an isometry (rigid-body
    // transformation).
    let isometry = Isometry3::from_parts(translation, rotation.into());
    println!("Isometry (translation + rotation):\n{isometry}");

    // Apply the isometry to a point.
    let transformed = isometry * point;
    println!("Transforming {point} yields: {transformed}");
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
