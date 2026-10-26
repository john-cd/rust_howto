// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `cc` crate to compile a C++ file as
//! part of a Rust build process.
//!
//! The package build script compiles `foo.cpp` as C++ and links the resulting
//! static library into the Rust targets. The book shows the exact build script
//! code and the bundled C++ source.

/// Runs the C++ binding that links the library built by `build.rs`.
fn main() {
    super::cc_bundled_cpp1::run();
}

// ANCHOR_END: example

/// Runs the C++ build and linkage example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the C++ library built by the script links and runs.
    #[test]
    fn bundled_cpp_library_runs() {
        main();
    }
}
