#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of the `glam` crate.
//!
//! `glam` is a linear algebra library for games and graphics applications.

use glam::Mat4;
use glam::Vec2;
use glam::Vec3;

fn main() {
    // Create some vectors.
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);

    // Vector addition.
    let v3 = v1 + v2;
    println!("v1 + v2 = {v3:?}");

    // Dot product.
    let dot_product = v1.dot(v2);
    println!("v1 . v2 = {dot_product:?}");

    // Cross product in 3D space.
    let v4 = Vec3::new(1.0, 2.0, 3.0);
    let v5 = Vec3::new(4.0, 5.0, 6.0);
    let cross_product = v4.cross(v5);
    println!("v4 x v5 = {cross_product:?}");

    // Create a 4x4 matrix.
    let mat = Mat4::from_scale_rotation_translation(
        Vec3::new(1.0, 2.0, 3.0),
        glam::Quat::from_rotation_y(45.0_f32.to_radians()),
        Vec3::new(4.0, 5.0, 6.0),
    );

    // Transform a vector using the matrix.
    let transformed_vector = mat.transform_point3(Vec3::new(1.0, 0.0, 0.0));
    println!("Transformed vector: {transformed_vector:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [finish](https://github.com/john-cd/rust_howto/issues/770)
