//! Build script to compile gRPC service definitions at compile time.
//!
//! TODO Reviewed ??-??-2026.

/// Compiles the gRPC service definitions.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instruct Cargo to re-run this script if a proto file changes.
    println!("cargo:rerun-if-changed=proto/helloworld.proto");
    println!("cargo:rerun-if-changed=proto/echo.proto");

    // Compile both schemas without launching an external protoc executable.
    // ANCHOR: protox_build
    for proto in ["proto/helloworld.proto", "proto/echo.proto"] {
        let descriptors = protox::compile([proto], ["proto"])?;
        tonic_prost_build::configure()
            .build_server(true)
            .compile_fds(descriptors)?;
    }
    // ANCHOR_END: protox_build
    Ok(())
}
