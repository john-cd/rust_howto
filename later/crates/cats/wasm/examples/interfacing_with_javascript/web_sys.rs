// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls a browser API from Rust through `web-sys`.
//!
//! `web-sys` provides bindings to the browser's APIs, such as the document, the
//! DOM (the tree of page elements) and the console, for Rust code compiled to
//! WebAssembly. The code must run in a browser, not as a normal program.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_sys::window;

/// Shows an alert in a browser or returns an error when no window is available.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn show_alert() -> Result<(), JsValue> {
    let window = window()
        .ok_or_else(|| JsValue::from_str("No browser window is available"))?;
    window.alert_with_message("Hello from Rust!")?;
    Ok(())
}
// ANCHOR_END: example

/// Explains how to run the browser-only example on a native host.
fn main() {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    println!(
        "Build for wasm32-unknown-unknown to call the browser Window API."
    );
}

/// Runs the example's host-side runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the host-side example runner succeeds.
    /// Runs the web sys example through its entry point.
    // output-only: on the host, the example only prints a hint; the wasm32 code
    // needs a browser.
    #[test]
    fn test_web_sys_example_entry_point() {
        main();
    }
}
