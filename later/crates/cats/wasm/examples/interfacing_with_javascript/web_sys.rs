#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calling browser APIs from Rust with `web-sys`.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Shows an alert box with the browser's `window.alert`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn show_alert() {
    if let Some(window) = window() {
        window
            .alert_with_message("Hello from Rust!")
            .expect("Failed to show alert");
    }
}

/// Does nothing outside of a browser; the function above runs in WebAssembly.
fn main() {
    // TODO
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
