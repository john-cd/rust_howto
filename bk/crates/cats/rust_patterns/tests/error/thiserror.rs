#![allow(dead_code)]
// ANCHOR: example
// Import the `Error` derive macro from the `thiserror` crate.
use thiserror::Error;

/// Define a custom error type `DataStoreError` using the `thiserror` crate.
/// The `Error` derive macro automatically implements the `std::error::Error`
/// trait.
#[derive(Error, Debug)]
pub enum DataStoreError {
    // A `Display` impl is generated for your error if you provide
    // #[error("...")] messages on the struct or each variant of your enum.
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error), /* A `From` impl is generated for
                                         * each variant containing
                                         * a #[from] attribute. */

    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("unknown data store error")]
    Unknown,

    #[error(transparent)]
    // Forward the source and Display methods straight through to an
    // underlying error without adding an additional message.
    Other(#[from] anyhow::Error),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For demonstration purposes, return an error of type
    // `DataStoreError::Unknown`.
    Err(DataStoreError::Unknown)?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    let res = main();
    println!("{:?}", res);
    assert!(res.is_err())
}
