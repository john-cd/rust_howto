// // ANCHOR: example
// #![allow(non_snake_case)]

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
// // TODO P1 write; add to markdown; https://github.com/mozilla/uniffi-rs/blob/main/examples/arithmetic/Cargo.toml
