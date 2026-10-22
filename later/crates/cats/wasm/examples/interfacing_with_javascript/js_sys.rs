#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Calling JavaScript from Rust with `js-sys`.

#[cfg(target_arch = "wasm32")]
use js_sys::Date;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Returns the current time, as a string, from JavaScript's `Date`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_current_time() -> String {
    let date = Date::new_0();
    String::from(date.to_utc_string())
}

/// Does nothing yet.
fn main() {} // TODO
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
