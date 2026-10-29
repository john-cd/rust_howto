// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Uses a JavaScript built-in from Rust through `js-sys`.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use js_sys::Date;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

/// Returns the current JavaScript date as a string on the browser target.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn get_current_time() -> String {
    let date = Date::new_0();
    String::from(date.to_utc_string())
}
// ANCHOR_END: example

/// Explains how to run the browser-only example on a native host.
fn main() {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    println!("Build for wasm32-unknown-unknown to call JavaScript's Date API.");
}

/// Runs the example's host-side runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the host-side example runner succeeds.
    /// Runs the js sys example through its entry point.
    // output-only: on the host, the example only prints a hint; the wasm32 code
    // needs a browser.
    #[test]
    fn test_js_sys_example_entry_point() {
        main();
    }
}
