#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: capnp
//! Entry point for the build script.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

/// Compiles the Cap'n Proto schema or copies its checked-in generated source.
///
/// # Errors
///
/// Returns an error if the compiler fails or the generated source cannot be
/// copied into Cargo's output directory.
fn capnp() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-env-changed=PATH");
    println!("cargo::rerun-if-changed=examples/binary_encoders/foo.capnp");
    println!("cargo::rerun-if-changed=examples/binary_encoders/foo_capnp.rs");

    match Command::new("capnp").arg("--version").output() {
        Ok(_) => {
            capnpc::CompilerCommand::new()
                .src_prefix("examples/binary_encoders")
                .file("examples/binary_encoders/foo.capnp")
                .run()?;
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            let generated_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples/binary_encoders/foo_capnp.rs");
            let output_path = PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| {
                std::io::Error::new(
                    ErrorKind::NotFound,
                    "Cargo did not set OUT_DIR for the Cap'n Proto generated source",
                )
            })?)
            .join("foo_capnp.rs");
            fs::copy(generated_path, output_path)?;
            println!(
                "cargo::warning=capnp compiler not found; using checked-in generated code"
            );
        }
        Err(error) => return Err(Box::new(error)),
    }

    Ok(())
}
// ANCHOR_END: capnp

// ANCHOR: prost
/// Compiles the Protocol Buffers schema and generates its Rust types.
///
/// # Errors
///
/// Returns an error if parsing the schema or generating Rust code fails.
fn prost() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=examples/binary_encoders/person.proto");

    let descriptors = protox::compile(
        ["examples/binary_encoders/person.proto"],
        ["examples/binary_encoders/"],
    )?;
    prost_build::compile_fds(descriptors)?;

    Ok(())
}
// ANCHOR_END: prost

/// Runs the schema compilers needed by this crate's examples.
///
/// # Errors
///
/// Returns an error if Protocol Buffers code generation fails.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnp()?;
    prost()?;
    Ok(())
}
// is prost fixed?
