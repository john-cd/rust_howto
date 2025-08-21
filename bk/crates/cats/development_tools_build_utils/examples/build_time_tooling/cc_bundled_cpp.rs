#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `cc` crate to compile a C++ file as
//! part of a Rust build process.
//!
//! The `cc` crate is a build dependency that allows you to compile C/C++ code
//! and link it into your Rust project. This is useful when you need to
//! interface with existing C/C++ libraries or when you have
//! performance-critical code that you want to write in C++.
//!
//! In this example, we compile a simple C++ file named `src/foo.cpp` and link
//! it into our Rust project. The compiled library will be named `foo`.
fn main() {
    // Create a new `cc::Build` instance.
    cc::Build::new()
        // Indicate that we are compiling C++ code.
        .cpp(true)
        // Specify the C++ file to compile.
        .file("src/foo.cpp")
        // Compile the C++ file and name the resulting library "foo".
        .compile("foo");
}
// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [FIX finish; deal with cc](https://github.com/john-cd/rust_howto/issues/897)
