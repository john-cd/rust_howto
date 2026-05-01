#![allow(dead_code)]
// ANCHOR: example
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

fn main() {
    // Construct a C++ string.
    let_cxx_string!(name = "Rust");
    // Call a C++ function.
    // ffi::hello(&name); // Requires actual C++ library to be linked
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
// [finish](https://github.com/john-cd/rust_howto/issues/738)
