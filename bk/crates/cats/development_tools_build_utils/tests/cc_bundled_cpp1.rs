// #![allow(dead_code)]
// // ANCHOR: example
// /// External block for declaring function interfaces that Rust code can call
// /// foreign code by. This is a declaration of a foreign function interface
// /// (FFI).
// ///
// /// `unsafe` indicates that calling this function is unsafe because the Rust
// /// compiler cannot guarantee the safety of the code in the foreign library.
// ///
// /// `extern "C"` specifies the calling convention. "C" is the most common
// /// calling convention for interoperability with C and C++ code.
// unsafe extern "C" {
//     /// Declares a function `multiply` that takes two 32-bit integers and
// returns a 32-bit integer.     /// This function is defined in a foreign
// library (C/C++).     fn multiply(x: i32, y: i32) -> i32;
// }

// fn main() {
//     unsafe {
//          println!("{}", multiply(5, 7));
//     }
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/898)
