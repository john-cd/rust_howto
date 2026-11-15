//! Runs the host-side part of the WebAssembly development example.
//!
//! TODO Reviewed ??-??-2026.

/// Exports a simple Rust function to JavaScript when built for the browser
mod wasm;
/// Runs the example through the native host runner.
fn main() {
    wasm::run();
}

/// Verifies that the host-side part of the WebAssembly development example.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    // output-only: the module prints its result.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
