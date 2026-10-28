// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls C functions that return preprocessor definitions supplied by `cc`.

use std::ffi::CStr;
use std::ffi::c_char;

unsafe extern "C" {
    /// Returns the application name defined for the C source.
    fn app_name() -> *const c_char;
    /// Returns the package version defined for the C source.
    fn app_version() -> *const c_char;
    /// Reports whether the `WELCOME` preprocessor macro was defined.
    fn welcome_is_defined() -> i32;
}

/// Reads the values compiled into the C library.
fn read_app_info() -> (String, String, bool) {
    // SAFETY: the C functions return null-terminated string literals from
    // macros defined in `cc_defines.c`; those literals live for the program's
    // duration. `welcome_is_defined` returns an integer and accepts no pointer.
    unsafe {
        (
            CStr::from_ptr(app_name()).to_string_lossy().into_owned(),
            CStr::from_ptr(app_version()).to_string_lossy().into_owned(),
            welcome_is_defined() != 0,
        )
    }
}

/// Prints the C preprocessor values supplied by the build script.
fn main() {
    let (app_name, version, welcome) = read_app_info();
    let greeting = if welcome {
        "Welcome to"
    } else {
        "Application:"
    };
    println!("{greeting} {app_name} - version {version}");
}

// ANCHOR_END: example

/// Runs the C preprocessor definitions example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the C library receives all three build-time definitions.
    #[test]
    fn build_script_defines_are_visible_to_c() {
        assert_eq!(
            read_app_info(),
            ("foo".to_owned(), env!("CARGO_PKG_VERSION").to_owned(), true)
        );
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
