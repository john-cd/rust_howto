//! Build script to compile gRPC service definitions at compile time.
//!
//! TODO Reviewed ??-??-2026.

/// Compiles the gRPC service definitions.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instruct Cargo to re-run this script if a proto file changes.
    println!("cargo:rerun-if-changed=proto/helloworld.proto");
    println!("cargo:rerun-if-changed=proto/echo.proto");
    println!("cargo:rerun-if-env-changed=PROTOC");

    // Compile the schema and generate the gRPC service code without launching
    // an external protoc executable.
    let descriptors = protox::compile(["proto/helloworld.proto"], ["proto"])?;
    tonic_prost_build::compile_fds(descriptors)?;

    compile_with_vendored_protoc()?;
    Ok(())
}

// ANCHOR: vendored_protoc
/// Compiles `proto/echo.proto` with a prebuilt `protoc` from
/// `protoc-bin-vendored`, unless the user already set `PROTOC`.
fn compile_with_vendored_protoc() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("PROTOC").is_none() {
        // SAFETY: a build script is single-threaded at this point.
        unsafe {
            std::env::set_var(
                "PROTOC",
                protoc_bin_vendored::protoc_bin_path()?,
            );
        }
    }
    tonic_prost_build::configure()
        .build_server(true)
        .compile_protos(&["proto/echo.proto"], &["proto"])?;
    Ok(())
}
// ANCHOR_END: vendored_protoc
