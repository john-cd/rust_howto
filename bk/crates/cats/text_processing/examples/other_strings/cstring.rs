#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to work with C-style strings (`CString` and
//! `CStr`) in Rust, which are essential for FFI (Foreign Function Interface)
//! interactions.

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

// Example external function that takes a C string
// (from the standard C library).
unsafe extern "C" {
    // It accepts a raw pointer to a C-style string,
    // which must be terminated by \0 (`nul`).
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    // 1. Call a FFI function that requires a C string.

    // Create a CString (owned nul-terminated, C-compatible string) from a Rust
    // `&str`:
    let rust_string = "Hello, C world!";
    let c_string = CString::new(rust_string).expect("CString creation failed");

    // Get the raw pointer.
    let raw_ptr: *const c_char = c_string.as_ptr();

    // Call a C function with the raw pointer. Note the unsafe block.
    let len = unsafe { strlen(raw_ptr) };
    println!("String length according to C: {}", len);

    // 2. Work with null-terminated strings from C.

    // Simulate a string received from C code. Note the `nul` terminator.
    // In real code, this would come from an external C function call.
    let c_hello = b"Hello\0" as *const u8 as *const c_char;
    let borrowed: &CStr;
    unsafe {
        // Wrap the pointer in a CStr (borrowed C string slice).
        borrowed = CStr::from_ptr(c_hello);
    }

    // Convert to Rust `&str`.
    let rust_str: &str = borrowed.to_str().expect("Invalid UTF-8");
    println!("From C: {}", rust_str);

    // Create owned version of the C-style String.
    let owned: CString = CString::from(borrowed);
    println!("Owned: {:?}", owned);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
