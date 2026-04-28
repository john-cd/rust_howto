#![allow(dead_code)]
// ANCHOR: example
// Framework for building pre-compiled Node.js addons in Rust.

/// Import the preludes:
// use napi_derive::napi;

// #[napi] // Indicates that the function is exposed to JavaScript.
fn hello(name: String) -> String {
    format!("Hello, {name}!")
}

fn main() {}

#[test]
fn test() {
    main();
}
// ANCHOR_END: example

// [finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template](https://github.com/john-cd/rust_howto/issues/1032)
