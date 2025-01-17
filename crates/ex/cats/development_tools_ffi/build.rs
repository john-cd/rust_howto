// ANCHOR: BINDGEN
use std::env;
use std::path::PathBuf;

fn bindgen() {
    // The path to the header file
    let header = "tests/example.h";
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
//     cxx_build::bridge("tests/cxx.rs")
//         .file("tests/hello.cc")
//         .flag_if_supported("-std=c++14")
//         .compile("cxxbridge-demo");

//     println!("cargo:rerun-if-changed=tests/cxx.rs");
//     println!("cargo:rerun-if-changed=tests/hello.h");
//     println!("cargo:rerun-if-changed=tests/hello.cc");
// }
// ANCHOR_END: CXX

fn main() {
    bindgen();
    cbindgen();
    // cxx();
}
// TODO P0 fix
