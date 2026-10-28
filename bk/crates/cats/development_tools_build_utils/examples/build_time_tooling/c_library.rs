// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls a C function compiled and linked by the package build script.

use std::ffi::c_int;

unsafe extern "C" {
    /// Adds two integers in the bundled C library.
    fn add(left: c_int, right: c_int) -> c_int;
}

/// Prints the sum returned by the bundled C function.
fn main() {
    // SAFETY: `build.rs` compiles a matching `add` implementation from C.
    let sum = unsafe { add(20, 22) };
    println!("20 + 22 = {sum}");
}
// ANCHOR_END: example

/// Runs the C library example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::add;

    /// Verifies that the bundled C library is linked and callable.
    #[test]
    fn bundled_c_function_adds_integers() {
        // SAFETY: `build.rs` compiles a matching `add` implementation from C.
        assert_eq!(unsafe { add(19, 23) }, 42);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
