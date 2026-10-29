//! Runs the host-side part of the WebAssembly development example.
//!
//! TODO Reviewed ??-??-2026.

mod wasm;

fn main() {
    wasm::run();
}

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
