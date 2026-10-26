// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `cc` crate to compile a C file into
//! a static library.

/// Verifies that the build script's compiled C library can be called.
fn main() -> anyhow::Result<()> {
    super::cc_bundled_static1::run()
}

// ANCHOR_END: example

/// Runs the C build and linkage example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the bundled C static library links and runs.
    ///
    /// # Errors
    ///
    /// Returns an error if the C function call fails.
    #[test]
    fn bundled_library_runs() -> anyhow::Result<()> {
        main()
    }
}
