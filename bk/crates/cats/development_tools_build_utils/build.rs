//! Build script examples for `development_tools_build_utils`.
//!
//! See <https://doc.rust-lang.org/cargo/reference/build-scripts.html>.
#![allow(dead_code)]

// ANCHOR: C
/// Compile a C file into a static library.
fn c() {
    println!("cargo::rerun-if-changed=examples/hello.c");
    // Compile `examples/hello.c` into `libhello.a`:
    cc::Build::new().file("examples/hello.c").compile("hello");
}
// ANCHOR_END: C

// ANCHOR: CPP
/// Compile a C++ file into a static library.
fn cpp() {
    // Compile `examples/foo.cpp` into `libfoo.a`
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=examples/foo.cpp");
    cc::Build::new()
        .cpp(true)
        .file("examples/foo.cpp")
        .compile("foo");
}
// ANCHOR_END: CPP

// ANCHOR: C_DEFINES
/// Compile a C file with specific `#define` directives.
fn cc_defines() {
    println!("cargo::rerun-if-changed=examples/foo.c");
    // Compile `examples/foo.c` into `libfoo.a` with defines:
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", None)
        .file("examples/foo.c")
        .compile("foo");
}
// ANCHOR_END: C_DEFINES

/// Entry point for the build script.
fn main() {
    // c();
    // cc_defines();
    // cpp();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_functions_are_defined() {
        // Just assert they are valid syntax.
        // We cannot easily run them in a test environment without setting up
        // a dummy workspace or files.
        let _ = c as fn();
        let _ = cpp as fn();
        let _ = cc_defines as fn();
    }
}
