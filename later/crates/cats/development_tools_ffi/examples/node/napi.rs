#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calling Rust from Node.js with `napi`.

// Framework for building pre-compiled Node.js addons in Rust.

// Import the preludes:
// use napi_derive::napi;

// #[napi] // Indicates that the function is exposed to JavaScript.
/// Returns a greeting for `name`.
fn hello(name: String) -> String {
    format!("Hello, {name}!")
}

/// Does nothing yet; the function above is called from Node.js.
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

    /// Runs the napi example through its entry point.
    #[test]
    fn test_napi_example_entry_point() {
        main();
    }
}
// [finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template](https://github.com/john-cd/rust_howto/issues/1032)
