#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: capnp
//! Entry point for the build script.

/// Compile a Captain Proto schema.
fn capnp() {
    println!("cargo::rerun-if-changed=examples/binary_encoders/foo.capnp");
    capnpc::CompilerCommand::new()
        //.src_prefix("schema") //  For all files specified for compilation that start with prefix, removes the prefix when computing output filenames.
        .file("examples/binary_encoders/foo.capnp") // Add a file to the list of files to be compiled.
        .run()
        .expect("schema compiler command"); // TODO
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
    #[cfg(target_os = "linux")]
    capnp();
    prost()?;
    Ok(())
}
// [is prost fixed?](https://github.com/john-cd/rust_howto/issues/1417)
