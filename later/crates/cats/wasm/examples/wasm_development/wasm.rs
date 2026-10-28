// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Exports a simple Rust function to JavaScript when built for the browser
//! target.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

/// Adds two integers and exposes the function to JavaScript on the browser
/// target.
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen)]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
// ANCHOR_END: example

/// Runs the example on a native host.
pub fn run() {
    println!("2 + 2 = {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that addition works in the example.
    #[test]
    fn adds_two_integers() {
        assert_eq!(add(2, 2), 4);
    }
}
