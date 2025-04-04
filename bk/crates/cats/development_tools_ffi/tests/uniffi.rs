// ANCHOR: example
//#![allow(non_snake_case)]

// // This example demonstrates how to use UniFFI to create a Rust library that
// // can be called from other languages.

// use uniffi::Bindgen;

// #[uniffi::export]
// pub fn greet(name: String) -> String {
//     format!("Hello, {}!", name)
// }

// #[uniffi::export] // marks as available for FFI.
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// // ANCHOR_END: example

// // #[test]
// // fn test() {
// //     main();
// // }
// [finish; https://github.com/mozilla/uniffi-rs/blob/main/examples/arithmetic/Cargo.toml](https://github.com/john-cd/rust_howto/issues/1037)
