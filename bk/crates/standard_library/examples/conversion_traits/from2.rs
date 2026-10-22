#![allow(unused)]
// ANCHOR: example
//! Converting Custom Errors.
//!
//! TODO Reviewed ??-??-2026

use std::io;

/// An error that wraps errors of several types.
#[derive(Debug)]
pub(crate) enum MyError {
    /// An I/O error.
    Io(io::Error),
    /// An error message.
    Message(String),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<&str> for MyError {
    fn from(msg: &str) -> Self {
        Self::Message(msg.to_owned())
    }
}

/// Converts errors of different types into `MyError` with the `?` operator.
fn main() -> Result<(), MyError> {
    println!("Testing custom error conversion with the ? operator:");
    let _file = std::fs::File::open("missing.txt")?; // `io::Error` becomes `MyError`.
    return Err("custom failure".into()); // `&str` becomes `MyError`.
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), MyError> {
    main()
}

/// Checks that the conversions make `main` return an error.
fn test_main() {
    assert!(main().is_err());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert!(main().is_err());
    }
}
