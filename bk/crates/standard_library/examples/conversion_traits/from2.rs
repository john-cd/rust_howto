// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Converting custom errors from different sources.

use std::fmt;
use std::io;

/// An error that wraps errors of several types.
#[derive(Debug)]
pub(crate) enum MyError {
    /// An I/O error.
    Io(io::Error),
    /// An error message.
    Message(String),
}

impl fmt::Display for MyError {
    /// Formats the error for user-facing output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Message(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    /// Wraps an I/O error.
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<&str> for MyError {
    /// Wraps a string slice as an owned message.
    fn from(message: &str) -> Self {
        Self::Message(message.to_owned())
    }
}

/// Returns a sample I/O error to demonstrate conversion with `?`.
fn io_operation() -> Result<(), io::Error> {
    Err(io::Error::other("sample I/O failure"))
}

/// Converts the I/O error into `MyError` with the `?` operator.
fn io_operation_with_custom_error() -> Result<(), MyError> {
    io_operation()?;
    Ok(())
}

/// Demonstrates conversions from I/O errors and string messages.
fn main() -> Result<(), MyError> {
    let io_error = io_operation_with_custom_error()
        .expect_err("the sample operation fails");
    assert!(matches!(&io_error, MyError::Io(_)));
    println!("{io_error}");

    let message_error: MyError = "custom failure".into();
    assert_eq!(message_error.to_string(), "custom failure");
    println!("{message_error}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), MyError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both error conversions are exercised by the successful
    /// example path.
    /// Runs the from2 example through its entry point.
    #[test]
    fn test_from2_example_entry_point() {
        assert!(main().is_ok());
    }
}
