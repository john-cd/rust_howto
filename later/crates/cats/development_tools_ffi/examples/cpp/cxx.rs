#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Calling C++ from Rust with the `cxx` crate.

// use cxx::CxxString;
use cxx::let_cxx_string;

/// Call a C++ function from Rust.
///
/// `cxx` provides a safe mechanism for calling C++ code from Rust and Rust
/// code from C++.
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("hello.h");

        // fn hello(name: &CxxString);
    }
}

/// Creates a C++ string to pass to C++.
fn main() {
    // Construct a C++ string.
    let_cxx_string!(name = "Rust");
    // Call a C++ function.
    // ffi::hello(&name); // Requires actual C++ library to be linked
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/738)
