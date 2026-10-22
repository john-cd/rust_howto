#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of the `thiserror` crate for creating custom error
//! types. This example shows how to wrap an underlying `std::io::Error` and
//! customize its display.
//!
//! TODO Reviewed ??-??-2026

use thiserror::Error;

/// A custom error type that wraps an underlying `std::io::Error`.
#[derive(Error, Debug)]
pub struct MyError {
    /// A message that describes the error.
    msg: String,

    // The `Error` trait's `source()` method is implemented to return whichever
    // field has a `#[source]` attribute or is named `source`, if any. This is
    // for identifying the underlying lower level error that caused your
    // error. `#[from]` implies `#[source]`. Any error type that implements
    // `std::error::Error` or dereferences to `dyn std::error::Error` will work
    // as a source.
    /// The underlying I/O error.
    #[source]
    source: std::io::Error,
    // Automatically detected to implement `provide()`:
    // backtrace: std::backtrace::Backtrace,
}

/// Implement the `Display` trait for `MyError` to customize its string
/// representation.
impl std::fmt::Display for MyError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

/// An example function that demonstrates how to create and return a `MyError`.
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let io_error = std::io::Error::other("oh no!");
    Err(Box::new(MyError {
        msg: "Error message".to_owned(),
        source: io_error,
    }))
}

/// Calls a function that fails and prints the outcome.
fn main() {
    match example() {
        Ok(()) => {
            println!("Got OK");
        }
        Err(err) => {
            println!("Got {err}");
        }
    }
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
