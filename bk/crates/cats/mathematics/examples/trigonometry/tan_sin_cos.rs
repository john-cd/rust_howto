#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates that the tangent of a number is equal to the sine
//! of the number divided by the cosine of the number.

/// This example demonstrates that the tangent of a number is equal to the sine
/// of the number divided by the cosine of the number.
fn main() {
    // Define a number.
    let x: f64 = 6.0;

    // Calculate the tangent of the number.
    let a = x.tan();
    // Calculate the sine of the number divided by the cosine of the number.
    let b = x.sin() / x.cos();

    println!("a: {a}, b: {b}");
    assert!((a - b).abs() < 1e-10, "tan(x)={a}, sin(x)/cos(x)={b}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tan sin cos example through its entry point.
    #[test]
    fn test_tan_sin_cos_example_entry_point() {
        main();
    }
}
