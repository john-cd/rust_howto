#![allow(dead_code)]
// ANCHOR: example
//! Build and inspect basic X.509 certificate data with the `x509_cert` crate.
//!
//! The example demonstrates DER encoding of names, serial numbers, and
//! validity periods.
//!
//! TODO Reviewed ??-??-2026

use std::str::FromStr;
use std::time::Duration;

use x509_cert::der::Encode;
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::time::Validity;

/// Builds the subject, serial number and validity of an X.509 certificate and
/// prints their DER lengths.
fn main() -> anyhow::Result<()> {
    let subject = Name::from_str("CN=example.com,O=Rust HowTo,C=US")?;
    let serial_number = SerialNumber::from(42u32);
    let validity = Validity::from_now(Duration::from_secs(24 * 60 * 60))?;

    println!("Subject DER length: {}", subject.to_der()?.len());
    println!("Serial number: {serial_number}");
    println!("Validity DER length: {}", validity.to_der()?.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/705)
}
