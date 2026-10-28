// ANCHOR: example
//! Awaits a JavaScript Promise from Rust.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use js_sys::Promise;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen_futures::JsFuture;

/// Validates a value resolved from JavaScript as a string.
fn resolved_text(value: Option<String>) -> Result<String, &'static str> {
    value.ok_or("Promise did not resolve to a string")
}

/// Awaits a JavaScript Promise and returns its resolved string value.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub async fn await_text(promise: Promise) -> Result<String, JsValue> {
    let value = JsFuture::from(promise).await?;
    resolved_text(value.as_string()).map_err(JsValue::from_str)
}
// ANCHOR_END: example

/// Explains how to run the browser-only example on a native host.
fn main() {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    println!("Build for wasm32-unknown-unknown to await a JavaScript Promise.");
}

/// Runs the example's host-side runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::resolved_text;

    /// Verifies that a resolved JavaScript string is returned.
    #[test]
    fn accepts_resolved_text() {
        assert_eq!(
            resolved_text(Some(String::from("ready"))),
            Ok(String::from("ready"))
        );
    }

    /// Verifies that non-string Promise results are rejected.
    #[test]
    fn rejects_non_string_results() {
        assert_eq!(
            resolved_text(None),
            Err("Promise did not resolve to a string")
        );
    }

    /// Verifies that the host-side example runner succeeds.
    #[test]
    fn host_runner_succeeds() {
        super::main();
    }
}
