// ANCHOR: example
//! Generate a self-signed certificate for local development with `rcgen`.
//!
//! A certificate binds a public key to a name, such as a host name, and is
//! normally signed by a certificate authority (CA) that clients trust. A
//! self-signed certificate vouches for itself, so clients must be told to trust
//! it explicitly. It is handy for local development and tests, but never for
//! production.

use rcgen::generate_simple_self_signed;

/// Creates and prints a certificate for the `localhost` name.
fn main() -> anyhow::Result<()> {
    let certified_key =
        generate_simple_self_signed(vec!["localhost".to_owned()])?;

    // Print the public certificate only; do not print or commit the private
    // key.
    println!("{}", certified_key.cert.pem());
    Ok(())
}
// ANCHOR_END: example

/// Runs the self-signed certificate example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generates and prints a self-signed localhost certificate.
    #[test]
    fn generates_localhost_certificate() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn certificate_and_key_are_pem_encoded() -> anyhow::Result<()> {
        let key = generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let cert = key.cert.pem();
        assert!(cert.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(cert.trim_end().ends_with("-----END CERTIFICATE-----"));
        assert!(key.signing_key.serialize_pem().contains("PRIVATE KEY"));
        Ok(())
    }

    #[test]
    fn each_generated_certificate_is_unique() -> anyhow::Result<()> {
        let a = generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let b = generate_simple_self_signed(vec!["localhost".to_owned()])?;
        assert_ne!(a.cert.pem(), b.cert.pem());
        Ok(())
    }
}
