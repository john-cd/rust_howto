#![allow(dead_code)]
// TODO Reviewed ??-??-2026

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
    }
}

/// Creates a C++ string to pass to C++.
fn main() {
    // Construct a C++ string.
    let_cxx_string!(name = "Rust");
    // Call the C++ function compiled by the package build script.
    call_cpp(&name);
}

/// Passes a C++ string through the generated bridge.
fn call_cpp(name: &CxxString) {
    ffi::hello(name);
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls the C++ function through the generated bridge.
    #[test]
    fn test_main() {
        main();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/738)
