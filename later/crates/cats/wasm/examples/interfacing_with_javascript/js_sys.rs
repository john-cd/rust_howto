#![allow(dead_code)]
// ANCHOR: example
#[cfg(target_arch = "wasm32")]
use js_sys::Date;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_current_time() -> String {
    let date = Date::new_0();
    String::from(date.to_utc_string())
}
// ANCHOR_END: example

pub fn run() {
    main();
}
