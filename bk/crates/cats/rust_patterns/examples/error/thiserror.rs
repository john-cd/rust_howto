#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Defining an error type with `thiserror`.

// Import the `Error` derive macro from the `thiserror` crate.
use thiserror::Error;

/// Define a custom error type `DataStoreError` using the `thiserror` crate.
/// The `Error` derive macro automatically implements the `std::error::Error`
/// trait.
#[derive(Error, Debug)]
pub enum DataStoreError {
    // A `Display` impl is generated for your error if we provide
    // #[error("...")] messages on the struct or each variant of your enum.
    /// The data store disconnected.
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error), /* A `From` impl is generated for
                                         * each variant containing
                                         * a #[from] attribute. */

    /// The data for a key is not available.
    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    /// A header does not have the expected content.
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        /// The expected header.
        expected: String,
        /// The header that was found.
        found: String,
    },

    /// An unknown error of the data store.
    #[error("unknown data store error")]
    Unknown,

    /// Any other error, forwarded as is.
    #[error(transparent)]
    // Forward the source and Display methods straight through to an
    // underlying error without adding an additional message.
    Other(#[from] anyhow::Error),
}

/// Returns a `DataStoreError` on purpose.
#[expect(unreachable_code, reason = "the example returns an error on purpose")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For demonstration purposes, return an error of type
    // `DataStoreError::Unknown`.
    return Err(DataStoreError::Unknown.into());
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the example returns an error.
    /// Runs the thiserror example through its entry point.
    #[test]
    fn test_thiserror_example_entry_point() {
        let res = main();
        println!("{res:?}");
        assert!(res.is_err());
    }
}
