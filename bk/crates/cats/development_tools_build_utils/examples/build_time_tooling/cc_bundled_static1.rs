// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to call C functions from Rust.
//! It uses the `cc` crate to compile a C file into a static library,
//! and then links against that library.

use std::ffi::CString;
use std::os::raw::c_char;

use anyhow::Result;

unsafe extern "C" {
    /// Prints a greeting to the console.
    fn hello();
    /// Greets the supplied user.
    fn greet(name: *const c_char);
}

/// Calls the no-argument C greeting function.
fn hello_from_c() {
    // SAFETY: `build.rs` links the matching no-argument `hello` function.
    unsafe { hello() };
}

/// Greets a name through the bundled C function.
///
/// # Errors
///
/// Returns an error if `name` contains an interior NUL byte.
fn greet_from_c(name: &str) -> Result<()> {
    let name = CString::new(name)?;
    // SAFETY: `name` is NUL-terminated and remains alive for this call; the C
    // function reads it without retaining the pointer.
    unsafe { greet(name.as_ptr()) };
    Ok(())
}

/// Runs the C functions linked by the package build script.
fn main() -> Result<()> {
    hello_from_c();
    greet_from_c("Rust")
}

// ANCHOR_END: example

/// Runs the bundled C greeting example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the bundled C function accepts a valid C string.
    ///
    /// # Errors
    ///
    /// Returns an error if the C string is invalid.
    #[test]
    fn bundled_c_greeting_runs() -> Result<()> {
        greet_from_c("workspace")
    }

    /// Rejects a name that cannot be represented as a C string.
    #[test]
    fn rejects_interior_nul() {
        assert!(greet_from_c("not\0a C string").is_err());
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
