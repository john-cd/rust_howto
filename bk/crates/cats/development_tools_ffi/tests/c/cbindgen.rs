// ANCHOR: example
// // COMING SOON
// ANCHOR_END: example

// `cbindgen` is a useful tool for generating C headers from Rust code, which
// can be very helpful when you want to create a C API from a Rust library.

// 1) Add `cbindgen` as a build dependency in `Cargo.toml`:
// [build-dependencies]
// cbindgen = "0.27" # or latest version

// 2) Define some functions and types that you want to expose to C:

// The `no_mangle` attribute may be used on any item to disable standard symbol
// name mangling. The symbol for the item will be the identifier of the item's
// name. Additionally, the item will be publicly exported from the produced
// library or object file, similar to the `used` attribute.
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_point(x: f64, y: f64) -> Point {
    Point { x, y }
}

// 3) Create a `build.rs` file in your project's root directory to run
//    `cbindgen`.
// 4) Use `cargo build` to generate the bindings.
// 5) The generated C header file (bindings.h) will be located in the
//    target/debug/build/<project-name>-*/out/ directory.

// [fix SOON see build.rs](https://github.com/john-cd/rust_howto/issues/1002)
