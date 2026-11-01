// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Passing preprocessor definitions to C code compiled with the `cc` crate.
//!
//! Preprocessor definitions (`-DNAME=value`) switch parts of C code on or off
//! at compile time. The `cc` crate passes them from a build script, so Rust can
//! configure the native code it builds.

use std::ffi::CStr;
use std::ffi::c_char;

use anyhow::Context;
use anyhow::Result;

unsafe extern "C" {
    /// Returns the application name defined for the C source.
    fn app_name() -> *const c_char;
    /// Returns the package version defined for the C source.
    fn app_version() -> *const c_char;
    /// Reports whether the `WELCOME` preprocessor macro was defined.
    fn welcome_is_defined() -> i32;
}

/// Reads the values compiled into the C library.
///
/// # Errors
///
/// Returns an error if a C string cannot be converted to valid UTF-8.
fn read_app_info() -> Result<(String, String, bool)> {
    // SAFETY: the C functions return null-terminated string literals from
    // macros defined in `cc_defines.c`; those literals live for the program's
    // duration. `welcome_is_defined` returns an integer and accepts no pointer.
    let (name_ptr, version_ptr, welcome_raw) =
        unsafe { (app_name(), app_version(), welcome_is_defined()) };

    // SAFETY: `app_name` returns a non-null, NUL-terminated static string.
    let name = unsafe { CStr::from_ptr(name_ptr) }
        .to_str()
        .context("APP_NAME string is not valid UTF-8")?
        .to_owned();

    // SAFETY: `app_version` returns a non-null, NUL-terminated static string.
    let version = unsafe { CStr::from_ptr(version_ptr) }
        .to_str()
        .context("VERSION string is not valid UTF-8")?
        .to_owned();

    let welcome = welcome_raw != 0;

    Ok((name, version, welcome))
}

/// Prints the C preprocessor values supplied by the build script.
///
/// # Errors
///
/// Returns an error if reading C definitions fails.
fn main() -> Result<()> {
    let (app_name, version, welcome) = read_app_info()?;
    let greeting = if welcome {
        "Welcome to"
    } else {
        "Application:"
    };
    println!("{greeting} {app_name} - version {version}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the C preprocessor definitions example.
///
/// # Errors
///
/// Returns an error if reading C definitions fails.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the C library receives all three build-time definitions.
    ///
    /// # Errors
    ///
    /// Returns an error if reading C definitions fails.
    #[test]
    fn build_script_defines_are_visible_to_c() -> Result<()> {
        assert_eq!(
            read_app_info()?,
            ("foo".to_owned(), env!("CARGO_PKG_VERSION").to_owned(), true)
        );
        Ok(())
    }

    /// Verifies that `main` completes successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if `main` returns an error.
    #[test]
    fn test_main_entry_point() -> Result<()> {
        main()
    }
}
