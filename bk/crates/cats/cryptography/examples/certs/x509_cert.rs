#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build and inspect X.509 certificate fields with the `x509_cert` crate.
//!
//! This deterministic, offline example demonstrates DER encoding of a
//! distinguished name, serial number, and validity period. It builds the
//! certificate's data fields; it does not sign a certificate.

use std::str::FromStr as _;
use std::time::Duration;

use x509_cert::der::Encode as _;
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::time::Validity;

/// Builds the subject, serial number and validity of an X.509 certificate and
/// prints their DER lengths.
fn main() -> anyhow::Result<()> {
    let subject = Name::from_str("CN=example.com,O=Rust HowTo,C=US")?;
    let serial_number = SerialNumber::from(42u32);
    let validity = Validity::from_now(Duration::from_hours(24))?;

    println!("Subject DER length: {}", subject.to_der()?.len());
    println!("Serial number: {serial_number}");
    println!("Validity DER length: {}", validity.to_der()?.len());
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example's name, serial number, and validity encode as DER.
    #[test]
    fn certificate_fields_have_expected_der_encodings() -> anyhow::Result<()> {
        let subject = Name::from_str("CN=example.com,O=Rust HowTo,C=US")?;
        let serial_number = SerialNumber::from(42u32);
        let validity = Validity::from_now(Duration::from_hours(24))?;

        let subject_der = subject.to_der()?;
        assert_eq!(subject_der.first(), Some(&0x30));

        let serial_der = serial_number.to_der()?;
        assert_eq!(serial_der, [0x02, 0x01, 0x2a]);

        let validity_der = validity.to_der()?;
        assert_eq!(validity_der.first(), Some(&0x30));

        Ok(())
    }
}
