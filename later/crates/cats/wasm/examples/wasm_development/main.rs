//! Runs the host-side part of the WebAssembly development example.
//!
//! TODO Reviewed ??-??-2026.

/// Exports a simple Rust function to JavaScript when built for the browser
mod wasm;

/// Runs the example through the native host runner.
fn main() {
    wasm::run();
}
