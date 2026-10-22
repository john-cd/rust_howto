#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Encoding a struct to DER and decoding it back with the `der` crate.

// This example demonstrates DER encoding and decoding with the `der` crate.
use der::Decode as _;
use der::Encode as _;
use der::Sequence;

// This example demonstrates how to encode a struct to DER format, and decode it
// back using the `der` crate.

// Define a struct to be encoded/decoded
// Note that we derive the `Sequence` trait.
/// A sequence of two fields, encoded as a DER `SEQUENCE`.
#[derive(Debug, PartialEq, Sequence)]
struct MyStruct {
    /// The first field.
    field1: u32,
    /// The second field.
    field2: String,
}

/// Encodes a [`MyStruct`] to DER and decodes it back.
fn main() {
    // Create an instance of `MyStruct`:
    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello, DER!".to_owned(),
    };

    // Encode the struct to DER:
    let der_encoded = my_struct.to_der().expect("Failed to encode");
    println!("DER Encoded: {der_encoded:?}");

    // Decode the DER back to `MyStruct`:
    let decoded_struct =
        MyStruct::from_der(&der_encoded).expect("Failed to decode");
    println!("Decoded Struct: {decoded_struct:?}");

    // Verify that the decoded struct matches the original:
    assert_eq!(my_struct, decoded_struct);
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
