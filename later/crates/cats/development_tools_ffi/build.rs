// TODO Reviewed ??-??-2026

//! Generates the bindings and headers used by the C FFI examples.

use std::env;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

// ANCHOR: BINDGEN
/// Compiles the sample C library and generates Rust bindings for its header.
///
/// # Errors
///
/// Returns an error if Cargo omits a required directory, bindgen cannot parse
/// the header, or the generated bindings cannot be written.
fn bindgen(manifest_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let c_source = manifest_dir.join("examples/c/example.c");
    let header = manifest_dir.join("examples/c/example.h");

    cc::Build::new().file(&c_source).compile("example");

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy().into_owned())
        .generate()?;
    bindings.write_to_file(out_dir.join("bindings.rs"))?;
    Ok(())
}
// ANCHOR_END: BINDGEN

// ANCHOR: CBINDGEN
/// Generates a C header from the Rust C-ABI library example.
///
/// # Errors
///
/// Returns an error if cbindgen cannot parse the crate or the generated header
/// cannot be written.
fn cbindgen(manifest_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let bindings = cbindgen::Builder::new()
        .with_crate(manifest_dir)
        .generate()?;
    let mut header_contents = Vec::new();
    bindings.write(&mut header_contents);
    std::fs::write(out_dir.join("bindings.h"), header_contents)?;
    Ok(())
}
// ANCHOR_END: CBINDGEN

// For the CXX example:

// ANCHOR: CXX
// Code generator for constructing and compiling C++ code.
// fn cxx() {
//     cxx_build::bridge("examples/cpp/cxx.rs")
//         .file("examples/hello.cc")
//         .flag_if_supported("-std=c++14")
//         .compile("cxxbridge-demo");
//
//     println!("cargo:rerun-if-changed=examples/cpp/cxx.rs");
//     println!("cargo:rerun-if-changed=examples/cpp/hello.h");
//     println!("cargo:rerun-if-changed=examples/cpp/hello.cc");
// }
// ANCHOR_END: CXX

// ANCHOR: UNIFFI
// fn uniffi() {
//     uniffi::generate_bindings("examples/uniffi.rs", "my_library").unwrap();
// }
// ANCHOR_END: UNIFFI

/// Runs the FFI code generators for the C examples.
///
/// # Errors
///
/// Returns an error if Cargo does not provide its manifest or output
/// directory, or either generator fails.
fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    println!("cargo:rerun-if-changed=examples/c/example.c");
    println!("cargo:rerun-if-changed=examples/c/example.h");
    println!("cargo:rerun-if-changed=examples/c/cbindgen.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    bindgen(&manifest_dir, &out_dir)?;
    cbindgen(&manifest_dir, &out_dir)?;
    Ok(())
}
