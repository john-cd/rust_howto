#![allow(dead_code)]
// ANCHOR: example
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn show_alert() {
    if let Some(window) = window() {
        window
            .alert_with_message("Hello from Rust!")
            .expect("Failed to show alert");
    }
}

fn main() {
    // TODO
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
