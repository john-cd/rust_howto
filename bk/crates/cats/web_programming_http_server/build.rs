//! Build script to compile gRPC service definitions at compile time.
//!
//! TODO Reviewed ??-??-2026

/// Compiles the gRPC service definitions.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instruct Cargo to re-run this script if the proto file changes.
    println!("cargo:rerun-if-changed=proto/helloworld.proto");

    // Compile the schema and generate the gRPC service code without launching
    // an external protoc executable.
    let descriptors = protox::compile(["proto/helloworld.proto"], ["proto"])?;
    tonic_prost_build::compile_fds(descriptors)?;
    Ok(())
}
