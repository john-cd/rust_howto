// https://doc.rust-lang.org/cargo/reference/build-scripts.html
#![allow(dead_code)]

// ANCHOR: C
fn c() {
    cc::Build::new().file("src/hello.c").compile("hello");
    // outputs `libhello.a`
}
// ANCHOR_END: C
// ANCHOR: CPP
fn cpp() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/foo.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");
}
// ANCHOR_END: CPP
// ANCHOR: C_DEFINES
fn cc_defines() {
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", None)
        .file("src/foo.c")
        .compile("foo");
}
// ANCHOR_END: C_DEFINES
// ANCHOR: BINDGEN
use std::env;
use std::path::PathBuf;

fn bindgen() {
    // The path to the header file
    let header = "tests/development_tools_ffi/example.h";
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
fn cxx() {
    // `bridge` returns a `cc::Build` on which you should continue to set up any
    // additional source files or compiler flags, and lastly call its
    // `compile` method to execute the C++ build.
    cxx_build::bridge("tests/development_tools_ffi/cxx.rs")
        .file("tests/development_tools_ffi/hello.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=tests/development_tools_ffi/cxx.rs");
    println!("cargo:rerun-if-changed=tests/development_tools_ffi/hello.h");
    println!("cargo:rerun-if-changed=tests/development_tools_ffi/hello.cc");
}
// ANCHOR_END: CXX

fn main() {
    // c();
    // cc_defines();
    // cpp();
    bindgen();
    cbindgen();
    // cxx();
}
// TODO P0 fix
