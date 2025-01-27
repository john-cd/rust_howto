// // ANCHOR: example
// // `bindgen` allows you to generate Rust bindings for (existing) C
// // (and some C++) libraries.

// // 1. If needed, compile the C library
// // ```
// // gcc -c -o example.o example.c
// // ```
// // 1b. And create a static library from the object file
// // ```sh
// // ar rcs libexample.a example.o
// // ```
// // 2. Add `bindgen` to `Cargo.toml`:
// // ```toml
// // [build-dependencies]
// // bindgen = "0.71.1" # or latest version
// // ```
// // 3) Create a `build.rs` file in your project's root directory.
// // 4) Build the Rust project
// // ```sh
// // cargo build
// // ```
// // 5) Run the Rust project
// // ```sh
// // cargo run
// // ```

// extern crate example;
// use example::add;

// // Include the bindings.rs file generated by `bindgen`:
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// fn main() {
//     let a = 5;
//     let b = 10;
//     // Call the bound function:
//     let result = unsafe { add(a, b) };

//     println!("The result of adding {} and {} is {}", a, b, result);
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P0 fix](https://github.com/john-cd/rust_howto/issues/1001)
