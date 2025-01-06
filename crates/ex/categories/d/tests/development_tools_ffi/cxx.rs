// ANCHOR: example
use cxx::CxxString;

// Call a C++ function from Rust using the `cxx` crate

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("hello.h");

        fn hello(name: &CxxString);
    }
}

fn main() {
    let name = "Rust".into();
    ffi::hello(name);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/738)
