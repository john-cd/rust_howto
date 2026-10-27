#![allow(dead_code)]
// ANCHOR: example
//! Parse a DER-encoded X.509 certificate with `x509-cert`.
//!
//! `rcgen` supplies an offline certificate so this example does not depend on
//! a network server or a checked-in private key.

use rcgen::generate_simple_self_signed;
use x509_cert::Certificate;
use x509_cert::der::Decode as _;

/// Generates a local certificate and parses its DER structure.
fn main() -> anyhow::Result<()> {
    let certified_key =
        generate_simple_self_signed(vec!["localhost".to_owned()])?;
    let _certificate =
        Certificate::from_der(certified_key.cert.der().as_ref())?;

    println!("Parsed a DER-encoded X.509 certificate.");
    Ok(())
}
// ANCHOR_END: example

/// Runs the X.509 parsing example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses the locally generated X.509 certificate.
    #[test]
    fn parses_der_certificate() -> anyhow::Result<()> {
        main()
    }
}
