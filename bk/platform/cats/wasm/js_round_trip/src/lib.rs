//! Exports a small Rust API to JavaScript.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use wasm_bindgen::prelude::*;

/// Builds a greeting from a trimmed JavaScript string.
///
/// # Errors
///
/// Returns a JavaScript Error if the name is empty after trimming.
#[wasm_bindgen]
pub fn greet(name: &str) -> Result<String, JsError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(JsError::new("Name must not be empty"));
    }
    Ok(format!("Hello, {name}!"))
}
// ANCHOR_END: example
