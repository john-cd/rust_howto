#![allow(dead_code)]
// ANCHOR: example
//! Encode and decode PEM data with the `pem_rfc7468` crate.
//!
//! This example converts raw DER bytes into a PEM document and parses them
//! back to verify the round-trip.
//!
//! TODO Reviewed ??-??-2026

use pem_rfc7468::LineEnding;
use pem_rfc7468::decode_vec;
use pem_rfc7468::encode_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let der_data = vec![0x30, 0x03, 0x02, 0x01, 0x2a];
    let label = "TEST DATA";

    let pem = encode_string(label, LineEnding::default(), &der_data)?;
    let (decoded_label, decoded_der) = decode_vec(pem.as_bytes())?;

    println!("PEM document:\n{pem}");
    println!("Decoded label: {decoded_label}");
    println!("Decoded {} DER bytes", decoded_der.len());

    assert_eq!(label, decoded_label);
    assert_eq!(der_data, decoded_der);
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/697)
}
