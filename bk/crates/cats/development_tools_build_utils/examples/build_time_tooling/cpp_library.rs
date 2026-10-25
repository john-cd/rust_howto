// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calls a C++ function exported through the C ABI.

use std::ffi::c_int;

unsafe extern "C" {
    /// Multiplies two integers in the bundled C++ library.
    fn multiply(left: c_int, right: c_int) -> c_int;
}

/// Prints the product returned by the bundled C++ function.
fn main() {
    // SAFETY: `build.rs` compiles a matching `extern "C"` function from C++.
    let product = unsafe { multiply(6, 7) };
    println!("6 * 7 = {product}");
}
// ANCHOR_END: example

/// Runs the C++ library example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::multiply;

    /// Verifies that the bundled C++ function is linked and callable.
    #[test]
    fn bundled_cpp_function_multiplies_integers() {
        // SAFETY: `build.rs` compiles a matching `extern "C"` function from
        // C++.
        assert_eq!(unsafe { multiply(6, 7) }, 42);
    }
}
