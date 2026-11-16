//! Runs the host-side part of the WebAssembly development example.
//!
//! TODO Reviewed ??-??-2026.

/// Exports a simple Rust function to JavaScript when built for the browser
mod wasm;

/// Runs the example through the native host runner.
fn main() {
    wasm::run();
}

// ANCHOR: host-tests
/// Tests the exported arithmetic on the host.
#[cfg(test)]
mod tests {
    use super::wasm::add;

    /// Adds two integers.
    #[test]
    fn adds_two_integers() {
        assert_eq!(add(2, 2), 4);
    }
}
// ANCHOR_END: host-tests
