// ANCHOR: example
//! Imports a JavaScript function into Rust through `wasm-bindgen`.
//!
//! WebAssembly (Wasm) is a compact binary format that browsers run at
//! near-native speed. Wasm code cannot touch the page directly, so
//! `wasm-bindgen` generates the glue that lets Rust call JavaScript functions
//! and the other way round.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
unsafe extern "C" {
    /// Writes a message to the JavaScript console.
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

/// Writes a message through the browser's JavaScript console.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn log_message(message: &str) {
    // Imported foreign functions are unsafe to call.
    unsafe { log(message) };
}
// ANCHOR_END: example

/// Explains how to run the browser-only example on a native host.
fn main() {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    println!("Build for wasm32-unknown-unknown to import console.log.");
}

/// Runs the example's host-side runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    /// Verifies that the host-side example runner succeeds.
    // output-only: on the host, the example only prints a hint; the wasm32 code
    // needs a browser.
    #[test]
    fn host_runner_succeeds() {
        super::main();
    }
}
