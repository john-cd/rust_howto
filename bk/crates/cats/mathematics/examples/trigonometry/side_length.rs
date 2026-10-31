// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calculates the hypotenuse of a right triangle given an angle and the length
//! of the side opposite to it.
//!
//! In a right triangle, sine, cosine and tangent relate an angle to the ratios
//! of the sides. Given an angle and one side you can compute the others. Angles
//! in Rust's `f64` methods are in radians, not degrees.

/// Calculates the hypotenuse of a right triangle given an angle and the length
/// of the side opposite to it.
fn hypotenuse(angle: f64, side_length: f64) -> f64 {
    // Calculate the hypotenuse using the sine function.
    side_length / angle.sin()
}

/// Calculates the hypotenuse for a 1 radian angle opposite a side of 80.
fn main() {
    // The angle in radians.
    let angle: f64 = 1.0;
    // The length of the side opposite to the angle.
    let side_length = 80.0;

    let hypotenuse = hypotenuse(angle, side_length);

    println!("Hypotenuse: {hypotenuse}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the side length example through its entry point.
    #[test]
    fn test_side_length_example_entry_point() {
        main();
    }

    /// Known triangles: 30 degrees halves the hypotenuse; 90 degrees equals
    /// the side. A zero angle degenerates to infinity.
    #[test]
    fn computes_known_triangles() {
        assert!(
            (hypotenuse(std::f64::consts::FRAC_PI_6, 5.0) - 10.0).abs() < 1e-12
        );
        assert!(
            (hypotenuse(std::f64::consts::FRAC_PI_2, 7.0) - 7.0).abs() < 1e-12
        );
        assert!((hypotenuse(1.0, 80.0) - 95.07).abs() < 0.01);
        assert!(hypotenuse(0.0, 1.0).is_infinite());
    }
}
