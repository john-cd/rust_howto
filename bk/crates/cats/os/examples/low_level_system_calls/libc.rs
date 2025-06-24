#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use `libc` to call into the C standard
//! library from Rust.

use std::ffi::CStr;
use std::ffi::CString;
use std::io::Write;
use std::ptr;

use anyhow::anyhow;
// `libc` defines a large number of types, constants, and function headers.
// For example, C type definitions include:
use libc::c_char; // C char, which is not the same as a Rust `char`.
use libc::c_int; // C int.

// The function headers defined in `libc` make it unneccessary to manually
// declare the C functions we want to use in an "extern" block. Otherwise, we
// would have to write:
// unsafe extern "C" {
// // Declares an external C function `puts` that takes a pointer
// // to a `c_char` (a C string) and returns an `i32`:
//     fn puts(s: *const c_char) -> i32;
//     fn getenv(name: *const c_char) -> *const c_char;
//     fn strerror(errnum: c_int) -> *mut c_char;
//}

// 1. Using `puts` to print a string.
// In real life, use the Rust `print!` macro, of course.
fn print() {
    // C strings (nul-terminated array of bytes) are different from Rust
    // strings, thus we can't pass the latter directly to a C function.
    // We therefore use a `CString`, which represents an owned, C-compatible,
    // nul-terminated string with no nul bytes in the middle.
    let message = CString::new("Hello from Rust using libc!")
        .expect("CString::new failed");

    // Call the `puts` function inside an `unsafe` block, since interacting
    // with C code is considered unsafe in Rust.
    // `as_ptr` returns the inner pointer to this C string.
    unsafe {
        libc::puts(message.as_ptr());
    }
    std::io::stdout().flush().unwrap();
}

/// Helper function to convert a C string to a Rust String.
/// SAFETY: Among other requirements, the memory pointed to by `cstr_ptr` must
/// contain a valid nul terminator at the end of the string. See the
/// documentation of `CStr::from_ptr`.
unsafe fn cstr_to_rust_string(
    cstr_ptr: *const c_char,
) -> anyhow::Result<String> {
    if cstr_ptr.is_null() {
        return Err(anyhow!("C string pointer is null."));
    }
    unsafe {
        // Wraps a raw C string pointer with a safe C string wrapper.
        let cstr: &CStr = CStr::from_ptr(cstr_ptr);
        // Get a copy-on-write Cow<'_, str>, then guarantee a fresh
        // `String` allocation.
        Ok(String::from_utf8_lossy(cstr.to_bytes()).to_string())
    }
}

/// 2. Using `getenv` to get an environment variable.
fn env() {
    // We first create a `CStr` literal containing the env. variable name.
    // Note the `c` before the literal.
    let var_name: &CStr = c"PATH";

    // Call the `getenv` C function.
    let path_value: *mut c_char = unsafe { libc::getenv(var_name.as_ptr()) };
    // BEWARE: You need to take into account the lifetime of your `*const/mut
    // c_char` pointers and who owns them. Depending on the C API, you may
    // need to call a special deallocation function on the string.

    match unsafe { cstr_to_rust_string(path_value) } {
        Ok(value) => println!("PATH: {}", value),
        Err(err) => eprintln!("Error getting PATH: {}", err),
    }
}

/// 3. Using `strerror` to get a human-readable error message.
fn strerror() {
    // `strerror` takes a `c_int` as its input.
    // This is an invalid value in this example.
    let errnum: c_int = 22;
    let err_msg_ptr = unsafe { libc::strerror(errnum) };

    match unsafe { cstr_to_rust_string(err_msg_ptr) } {
        Ok(msg) => println!("\nError {}: {}", errnum, msg),
        Err(err) => eprintln!("Error getting error message: {}", err),
    }
}

/// 4. Example of handling a null pointer.
fn null_pointer() {
    let null_ptr: *const c_char = ptr::null();
    assert!(null_ptr.is_null());
    match unsafe { cstr_to_rust_string(null_ptr) } {
        Ok(_) => println!("This should not be printed"),
        Err(e) => println!("\nCaught expected error: {}", e),
    }
}

fn main() {
    print();
    env();
    strerror();
    null_pointer()
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
