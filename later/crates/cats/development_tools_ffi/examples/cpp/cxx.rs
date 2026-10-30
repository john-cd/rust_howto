// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling C++ from Rust with the `cxx` crate.

use cxx::CxxString;
use cxx::let_cxx_string;

// The package build script's native link directive does not reach this example
// target.
#[link(name = "development_tools_ffi_cxx", kind = "static")]
unsafe extern "C" {
    fn cxx_link_anchor();
}

/// Call a C++ function from Rust.
///
/// `cxx` provides a safe mechanism for calling C++ code from Rust and Rust
/// code from C++.
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("development_tools_ffi/examples/cpp/hello.h");

        fn hello(name: &CxxString);
        fn cpp_multiply(left: i32, right: i32) -> i32;
        fn cpp_add_via_rust(left: i32, right: i32) -> i32;
    }

    extern "Rust" {
        fn rust_add(left: i32, right: i32) -> i32;
    }
}

/// Adds two integers in Rust for the generated bridge.
fn rust_add(left: i32, right: i32) -> i32 {
    left + right
}

/// Creates a C++ string and exercises the Rust and C++ functions.
fn main() {
    // Reference the static C++ archive linked by the package build script.
    unsafe { cxx_link_anchor() };
    // Construct a C++ string.
    let_cxx_string!(name = "Rust");
    // Call the C++ function compiled by the package build script.
    call_cpp(&name);
    println!("6 * 7 = {}", ffi::cpp_multiply(6, 7));
    println!("40 + 2 = {}", ffi::cpp_add_via_rust(40, 2));
}

/// Passes a C++ string through the generated bridge.
fn call_cpp(name: &CxxString) {
    ffi::hello(name);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls the C++ function through the generated bridge.
    /// Runs the cxx example through its entry point.
    #[test]
    fn test_cxx_example_entry_point() {
        main();
        assert_eq!(ffi::cpp_multiply(6, 7), 42);
        assert_eq!(rust_add(40, 2), 42);
        assert_eq!(ffi::cpp_add_via_rust(40, 2), 42);
    }
}
