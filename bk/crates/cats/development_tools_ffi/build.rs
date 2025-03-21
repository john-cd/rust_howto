#![allow(dead_code)]
// ANCHOR: BINDGEN
use std::env;
use std::path::PathBuf;

fn bindgen() {
    // outputs `libexample.a` in target/debug/build/development_tools_ffi-.../
    cc::Build::new()
        .file("tests/c/example.c")
        .compile("example");

    // The path to the header file
    let header = "tests/c/example.h";
    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(header)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
// ANCHOR_END: BINDGEN

// ANCHOR: CBINDGEN
fn cbindgen() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("bindings.h");
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir);
}
// ANCHOR_END: CBINDGEN

// For `cxx.rs` example:

// ANCHOR: CXX
// Code generator for constructing and compiling C++ code.
// fn cxx() {
//     // `bridge` returns a `cc::Build` on which you should continue to set up
// any     // additional source files or compiler flags, and lastly call its
//     // `compile` method to execute the C++ build.
//     cxx_build::bridge("tests/cpp/cxx.rs")
//         .file("tests/hello.cc")
//         .flag_if_supported("-std=c++14")
//         .compile("cxxbridge-demo");

//     println!("cargo:rerun-if-changed=tests/cpp/cxx.rs");
//     println!("cargo:rerun-if-changed=tests/cpp/hello.h");
//     println!("cargo:rerun-if-changed=tests/cpp/hello.cc");
// }
// ANCHOR_END: CXX

// ANCHOR: UNIFFI
// fn uniffi() {
//     // Generate FFI bindings for the specified Rust functions.
//     // This will generate the FFI library in the target/debug directory
// (e.g., libmy_library.so on Linux, libmy_library.dylib on macOS).
//     uniffi::generate_bindings(
//         "tests/uniffi.rs", // Path to the Rust library file
//         "my_library", // Name of the generated FFI library
//     )
//     .unwrap();
// }
// ANCHOR_END: UNIFFI

fn main() {
    bindgen();
    cbindgen();
    // cxx();
    // uniffi();
}
// [fix build.rs for ffi SOON](https://github.com/john-cd/rust_howto/issues/1026)
