// ANCHOR: example
//! Read a PEM certificate file and decode its DER bytes.
//!
//! PEM is a text wrapper for binary cryptographic data: Base64 between
//! `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----` lines.
//! Decoding the Base64 gives the DER bytes, which parsers for certificates and
//! keys expect.

use std::fs::OpenOptions;
use std::fs::{self};
use std::io::Write as _;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use pem_rfc7468::decode_vec;
use rcgen::generate_simple_self_signed;

/// Reads one PEM certificate file, checks its label, and returns its DER bytes.
fn read_pem_certificate(path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
    let pem = fs::read(path)?;
    let (label, der) = decode_vec(&pem)?;
    anyhow::ensure!(
        label == "CERTIFICATE",
        "expected a CERTIFICATE PEM block, got {label}"
    );
    Ok(der)
}

/// Writes a temporary local certificate, reads it as PEM, then removes it.
fn main() -> anyhow::Result<()> {
    let certified_key =
        generate_simple_self_signed(vec!["localhost".to_owned()])?;
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rust_howto_certificate_{}_{}.pem",
        std::process::id(),
        nonce
    ));

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    let write_result = file.write_all(certified_key.cert.pem().as_bytes());
    drop(file);
    if let Err(error) = write_result {
        fs::remove_file(&path)?;
        return Err(error.into());
    }

    let read_result = read_pem_certificate(&path);
    fs::remove_file(&path)?;
    let der = read_result?;

    println!(
        "Read a CERTIFICATE PEM block containing {} DER bytes.",
        der.len()
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the PEM file-reading example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads and decodes a generated PEM certificate file.
    #[test]
    fn reads_certificate_file() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn reads_the_der_bytes_of_a_certificate_file() -> anyhow::Result<()> {
        let key = generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let dir = std::env::temp_dir()
            .join(format!("read_pem_test_{}", std::process::id()));
        fs::create_dir_all(&dir)?;
        let path = dir.join("cert.pem");
        fs::write(&path, key.cert.pem())?;
        let der = read_pem_certificate(&path)?;
        assert_eq!(der, key.cert.der().as_ref());
        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn other_pem_labels_are_rejected() -> anyhow::Result<()> {
        let key = generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let dir = std::env::temp_dir()
            .join(format!("read_pem_label_{}", std::process::id()));
        fs::create_dir_all(&dir)?;
        let path = dir.join("key.pem");
        fs::write(&path, key.signing_key.serialize_pem())?;
        let Err(err) = read_pem_certificate(&path) else {
            panic!("a private key is not a certificate");
        };
        assert!(err.to_string().contains("expected a CERTIFICATE"), "{err}");
        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn missing_and_malformed_files_are_errors() -> anyhow::Result<()> {
        assert!(read_pem_certificate("/definitely/not/here.pem").is_err());
        let dir = std::env::temp_dir()
            .join(format!("read_pem_bad_{}", std::process::id()));
        fs::create_dir_all(&dir)?;
        let path = dir.join("bad.pem");
        fs::write(&path, "not pem")?;
        assert!(read_pem_certificate(&path).is_err());
        fs::remove_dir_all(&dir)?;
        Ok(())
    }
}
