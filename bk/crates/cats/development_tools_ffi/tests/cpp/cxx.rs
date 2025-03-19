// #![allow(unused_imports)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use cxx::CxxString;
// use cxx::let_cxx_string;

// // `cxx` provides a safe mechanism for calling C++ code from Rust and Rust
// // code from C++.

// // Call a C++ function from Rust:

// #[cxx::bridge]
// mod ffi {
//     unsafe extern "C++" {
//         include!("d/tests/development_tools_ffi/hello.h");

//         fn hello(name: &CxxString);
//     }
// }

// fn main() {
//     let_cxx_string!(name = "Rust");
//     ffi::hello(&name);
// }

// #[test]
// fn test() {
//     main();
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/738)
