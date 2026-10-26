#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Encode and decode PEM data with the `pem_rfc7468` crate.
//!
//! This example converts raw DER bytes into a PEM document and parses them
//! back to verify the round-trip.

use pem_rfc7468::LineEnding;
use pem_rfc7468::decode_vec;
use pem_rfc7468::encode_string;

/// Encodes DER data as PEM, then decodes it back.
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example runs end to end.
    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        main()
    }

    /// The document has the RFC 7468 boundary lines and a base64 body.
    #[test]
    fn encoded_document_has_boundary_lines()
    -> Result<(), Box<dyn std::error::Error>> {
        let pem = encode_string(
            "TEST DATA",
            LineEnding::LF,
            &[0x30, 0x03, 0x02, 0x01, 0x2a],
        )?;
        assert_eq!(
            pem,
            "-----BEGIN TEST DATA-----\nMAMCASo=\n-----END TEST DATA-----\n"
        );
        Ok(())
    }

    /// Long input is wrapped at 64 characters per line.
    #[test]
    fn long_bodies_wrap_at_64_columns() -> Result<(), Box<dyn std::error::Error>>
    {
        let pem = encode_string("DATA", LineEnding::LF, &[7_u8; 100])?;
        let body: Vec<&str> =
            pem.lines().filter(|l| !l.starts_with("-----")).collect();
        assert!(body.len() > 1);
        assert!(body.iter().all(|l| l.len() <= 64));
        Ok(())
    }

    /// Text without boundary lines is not valid PEM.
    #[test]
    fn missing_boundaries_are_rejected() {
        assert!(decode_vec(b"MAMCASo=").is_err());
    }
}
