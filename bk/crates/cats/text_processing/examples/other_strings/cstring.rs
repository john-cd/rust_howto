// TODO Reviewed ??-??-2026.

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
    /// Returns the length of a null-terminated C string.
    fn strlen(s: *const c_char) -> usize;
}

/// Converts between Rust strings and C strings.
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
    println!("String length according to C: {len}");

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
    println!("From C: {rust_str}");

    // Create owned version of the C-style String.
    let owned: CString = CString::from(borrowed);
    println!("Owned: {owned:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the cstring example through its entry point.
    #[test]
    fn test_cstring_example_entry_point() {
        main();
    }

    /// C's `strlen` agrees with Rust on the length of a `CString`.
    #[test]
    fn strlen_matches_rust_length() {
        let c_string = CString::new("Hello, C world!").unwrap();
        // SAFETY: `c_string` is a valid NUL-terminated string.
        assert_eq!(unsafe { strlen(c_string.as_ptr()) }, 15);
        assert_eq!(c_string.as_bytes_with_nul().len(), 16);
    }

    /// An interior NUL byte is rejected.
    #[test]
    fn interior_nul_is_rejected() {
        let err = CString::new("a\0b").unwrap_err();
        assert_eq!(err.nul_position(), 1);
    }

    /// A borrowed C string converts to `&str` and to an owned `CString`.
    #[test]
    fn borrowed_cstr_round_trips() {
        let borrowed = c"Hello";
        assert_eq!(borrowed.to_str().unwrap(), "Hello");
        let owned = CString::from(borrowed);
        assert_eq!(owned.as_c_str(), borrowed);
    }
}
