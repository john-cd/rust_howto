// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Exporting a Rust API to other languages with `uniffi`.
//!
//! In a real project, this code lives in the `lib.rs` of a crate with
//! `crate-type = ["cdylib"]`; `uniffi-bindgen` then reads the compiled library
//! to generate Kotlin, Swift or Python bindings.

// Generates the C ABI scaffolding for every item marked `#[uniffi::export]`.
uniffi::setup_scaffolding!();

/// An error that crosses the language boundary as a native exception.
#[derive(Debug, PartialEq, Eq, uniffi::Error)]
#[uniffi(flat_error)]
#[non_exhaustive]
pub enum ArithmeticError {
    /// The result does not fit in a `u64`.
    IntegerOverflow,
}

impl std::fmt::Display for ArithmeticError {
    /// Formats the value for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntegerOverflow => write!(f, "integer overflow"),
        }
    }
}

/// Returns a greeting for `name`.
///
/// # Arguments
///
/// * `name` - Name included in the greeting.
///
/// # Returns
///
/// A greeting containing `name`.
#[uniffi::export]
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

/// Adds two numbers, failing instead of wrapping on overflow.
///
/// # Arguments
///
/// * `a` - First addend.
/// * `b` - Second addend.
///
/// # Returns
///
/// The sum when it fits in a `u64`.
///
/// # Errors
///
/// Returns [`ArithmeticError::IntegerOverflow`] if the sum exceeds `u64::MAX`.
#[uniffi::export]
pub fn add(a: u64, b: u64) -> Result<u64, ArithmeticError> {
    a.checked_add(b).ok_or(ArithmeticError::IntegerOverflow)
}

/// Prints a greeting and the sum of two numbers.
fn uniffi_example() -> Result<(), ArithmeticError> {
    println!("{}", greet("UniFFI".to_owned()));
    println!("{}", add(2, 3)?);
    Ok(())
}

/// Starts the UniFFI example as a binary.
///
/// # Errors
///
/// Returns an error if the checked addition overflows.
fn main() -> Result<(), ArithmeticError> {
    uniffi_example()
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> Result<(), ArithmeticError> {
    uniffi_example()
}

/// The exported functions can be called directly from Rust.
#[cfg(test)]
mod tests {
    use super::*;

    /// The exported functions can be called directly from Rust.
    #[test]
    fn exported_functions_work_from_rust() {
        assert_eq!(greet("World".to_owned()), "Hello, World!");
        assert_eq!(add(2, 3), Ok(5));
    }

    /// Overflow is reported as an error rather than wrapping.
    #[test]
    fn add_reports_overflow() {
        assert_eq!(add(u64::MAX, 1), Err(ArithmeticError::IntegerOverflow));
    }

    /// Runs the example through its entry point.
    #[test]
    fn test_uniffi_example_entry_point() -> Result<(), ArithmeticError> {
        uniffi_example()
    }

    /// Formats the caller's name in the greeting response.
    #[test]
    fn greet_formats_the_name() {
        assert_eq!(greet("Ada".to_owned()), "Hello, Ada!");
        assert_eq!(greet(String::new()), "Hello, !");
    }
}
