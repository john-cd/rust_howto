#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use `libc` to call into the C standard
//! library from Rust.

use std::ffi::CStr;
use std::ffi::CString;
use std::io::Write as _;
use std::ptr;

use anyhow::anyhow;
// `libc` defines a large number of types, constants, and function headers.
// For example, C type definitions include:
use libc::c_char; // C char, which is not the same as a Rust `char`.
use libc::c_int; // C int.

// The function declarations defined in `libc` make it unnecessary to manually
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
/// Prints a message by calling the C function `puts`.
fn print() -> anyhow::Result<()> {
    // C strings (nul-terminated array of bytes) are different from Rust
    // strings, thus we can't pass the latter directly to a C function.
    // We therefore use a `CString`, which represents an owned, C-compatible,
    // nul-terminated string with no nul bytes in the middle.
    let message = CString::new("Hello from Rust using libc!")?;

    // Call the `puts` function inside an `unsafe` block, since interacting
    // with C code is considered unsafe in Rust.
    // `as_ptr` returns the inner pointer to this C string.
    // SAFETY: `message` remains alive and NUL-terminated for the duration of
    // the call, and `puts` only reads the string.
    let result = unsafe { libc::puts(message.as_ptr()) };
    if result < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    std::io::stdout().flush()?;
    Ok(())
}

/// Helper function to convert a C string to a Rust String.
/// # Safety
///
/// If non-null, `cstr_ptr` must point to a readable allocation containing a
/// NUL terminator within that allocation. The pointed-to bytes must not be
/// mutated for the duration of this call. See `CStr::from_ptr`'s contract.
unsafe fn cstr_to_rust_string(
    cstr_ptr: *const c_char,
) -> anyhow::Result<String> {
    if cstr_ptr.is_null() {
        return Err(anyhow!("C string pointer is null."));
    }
    unsafe {
        // SAFETY: The caller guarantees that this non-null pointer addresses
        // a readable NUL-terminated string that remains valid for this call.
        // Wraps a raw C string pointer with a safe C string wrapper.
        let cstr: &CStr = CStr::from_ptr(cstr_ptr);
        // Get a copy-on-write Cow<'_, str>, then guarantee a fresh
        // `String` allocation.
        Ok(String::from_utf8_lossy(cstr.to_bytes()).to_string())
    }
}

/// 2. Using `getenv` to get an environment variable.
fn env() -> anyhow::Result<()> {
    // We first create a `CStr` literal containing the env. variable name.
    // Note the `c` before the literal.
    let var_name: &CStr = c"PATH";

    // Call the `getenv` C function.
    // SAFETY: `var_name` is a valid NUL-terminated string. The pointer is
    // converted to an owned Rust string immediately and is not retained.
    let path_value: *mut c_char = unsafe { libc::getenv(var_name.as_ptr()) };
    // BEWARE: You need to take into account the lifetime of the `*const/mut
    // c_char` pointers and who owns them. Depending on the C API, we may
    // need to call a special deallocation function on the string.

    // SAFETY: `getenv` returns either null or a NUL-terminated environment
    // value; conversion happens before another environment operation.
    let value = unsafe { cstr_to_rust_string(path_value) }?;
    println!("PATH: {value}");
    Ok(())
}

/// 3. Using `strerror` to get a human-readable error message.
fn strerror() -> anyhow::Result<()> {
    // `strerror` takes a `c_int` as its input.
    // This is an invalid value in this example.
    let errnum: c_int = 22;
    // SAFETY: `strerror` accepts any `c_int` and returns either a valid
    // NUL-terminated message pointer or null.
    let err_msg_ptr = unsafe { libc::strerror(errnum) };

    // SAFETY: `strerror`'s returned message is NUL-terminated and remains
    // valid until the next call to `strerror` in this thread.
    let msg = unsafe { cstr_to_rust_string(err_msg_ptr) }?;
    println!("\nError {errnum}: {msg}");
    Ok(())
}

/// 4. Example of handling a null pointer.
fn null_pointer() -> anyhow::Result<()> {
    let null_ptr: *const c_char = ptr::null();
    assert!(null_ptr.is_null());
    // SAFETY: null is explicitly handled before CStr::from_ptr is reached.
    let error = unsafe { cstr_to_rust_string(null_ptr) }
        .expect_err("a null C string pointer must be rejected");
    println!("\nCaught expected error: {error}");
    Ok(())
}

/// Calls the `libc` examples in turn.
fn main() -> anyhow::Result<()> {
    print()?;
    env()?;
    strerror()?;
    null_pointer()?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() {
    if let Err(error) = main() {
        eprintln!("libc example failed: {error:#}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the full libc example runs successfully.
    #[test]
    fn test_main() {
        main().expect("the libc example should complete successfully");
    }
}
