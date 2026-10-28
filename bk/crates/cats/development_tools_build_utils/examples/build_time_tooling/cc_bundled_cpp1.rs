// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling a C++ function compiled by a build script with the `cc` crate.

// Declare a C-compatible symbol that can be implemented in bundled C++ code.
unsafe extern "C" {
    /// Multiplies two integers in the bundled C++ implementation.
    fn multiply(x: i32, y: i32) -> i32;
}

/// Safely multiplies two integers with the bundled C++ function.
fn multiply_from_cpp(left: i32, right: i32) -> Option<i32> {
    let expected = left.checked_mul(right)?;
    // SAFETY: the build script links the matching implementation, and the
    // checked product ensures the C++ signed multiplication cannot overflow.
    let product = unsafe { multiply(left, right) };
    debug_assert_eq!(product, expected);
    Some(product)
}

/// Prints a product returned by the bundled C++ function.
fn main() {
    let product = multiply_from_cpp(6, 7);
    println!("6 * 7 = {product:?}");
}
// ANCHOR_END: example

/// Runs the bundled C++ multiplication example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies multiplication and rejection of overflowing inputs.
    #[test]
    fn bundled_cpp_function_multiplies_values() {
        assert_eq!(multiply_from_cpp(6, 7), Some(42));
        assert_eq!(multiply_from_cpp(i32::MIN, 0), Some(0));
        assert_eq!(multiply_from_cpp(i32::MAX, 2), None);
    }
}
