// TODO Reviewed ??-??-2026

//! Build script examples for `development_tools_build_utils`.
//!
//! See <https://doc.rust-lang.org/cargo/reference/build-scripts.html>.

// ANCHOR: C
/// Compile a C file into a static library.
fn c() {
    println!("cargo::rerun-if-changed=examples/build_time_tooling/hello.c");
    cc::Build::new()
        .file("examples/build_time_tooling/hello.c")
        .compile("hello");
}
// ANCHOR_END: C

// ANCHOR: CPP
/// Compile a C++ file into a static library.
fn cpp() {
    println!("cargo::rerun-if-changed=examples/build_time_tooling/foo.cpp");
    cc::Build::new()
        .cpp(true)
        .file("examples/build_time_tooling/foo.cpp")
        .compile("foo");
}
// ANCHOR_END: CPP

// ANCHOR: C_DEFINES
/// Compile a C file with specific `#define` directives.
fn cc_defines() {
    println!(
        "cargo::rerun-if-changed=examples/build_time_tooling/cc_defines.c"
    );
    let version = format!("\"{}\"", env!("CARGO_PKG_VERSION"));
    // Compile the C example with defines supplied by this Cargo package.
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define("VERSION", version.as_str())
        .define("WELCOME", None)
        .file("examples/build_time_tooling/cc_defines.c")
        .compile("cc_defines");
}
// ANCHOR_END: C_DEFINES

// ANCHOR: out-dir-codegen
/// Generates a Rust module from a tracked text input in Cargo's output
/// directory.
fn out_dir_codegen() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    const INPUT: &str = "examples/build_time_tooling/message.txt";

    println!("cargo::rerun-if-changed={INPUT}");
    let message = fs::read_to_string(INPUT)?;
    let generated = format!(
        "pub const GENERATED_MESSAGE: &str = {:?};\n",
        message.trim_end()
    );
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Cargo did not set OUT_DIR")
    })?;
    fs::write(
        PathBuf::from(out_dir).join("generated_message.rs"),
        generated,
    )?;
    Ok(())
}
// ANCHOR_END: out-dir-codegen

/// Entry point for the build script.
///
/// # Errors
///
/// Returns an error if generated source cannot be written to Cargo's output
/// directory.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: build-info
    build_info_build::build_script()
        .collect_dependencies(build_info_build::DependencyDepth::Depth(1));
    // ANCHOR_END: build-info

    c();
    cpp();
    cc_defines();
    out_dir_codegen()?;
    Ok(())
}
