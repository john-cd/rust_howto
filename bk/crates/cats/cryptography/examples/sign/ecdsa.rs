#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Sign and verify a message with ECDSA using the P-256 curve.
//!
//! This example uses `p256` and the `Signer`/`Verifier` traits. The signature
//! is serialized to its fixed-size byte form, as it would be to store or send
//! it, then parsed again before verification.

use p256::ecdsa::Signature;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::signature::Signer as _;
use p256::ecdsa::signature::Verifier as _;
use p256::elliptic_curve::rand_core::OsRng;

/// Signs a message with a random P-256 key and verifies the signature.
fn main() -> anyhow::Result<()> {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let message = b"Hello, world!";

    let signature: Signature = signing_key.sign(message);
    // A P-256 signature is two 32-byte integers, `r` and `s`.
    let bytes = signature.to_bytes();
    let parsed = Signature::from_slice(&bytes)?;
    verifying_key.verify(message, &parsed)?;

    // Any change to the message makes verification fail.
    assert!(verifying_key.verify(b"Hello, w0rld!", &parsed).is_err());

    println!(
        "Compressed public key bytes: {:?}",
        verifying_key.to_encoded_point(true).as_bytes()
    );
    println!(
        "Signature ({} bytes): {:02x?}",
        bytes.len(),
        bytes.as_slice()
    );
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example runs end to end.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    /// A signature made with one key does not verify under another key.
    #[test]
    fn wrong_key_is_rejected() {
        let signer = SigningKey::random(&mut OsRng);
        let other = VerifyingKey::from(&SigningKey::random(&mut OsRng));
        let signature: Signature = signer.sign(b"message");
        assert!(other.verify(b"message", &signature).is_err());
    }

    /// RFC 6979 makes signing deterministic: same key and message, same
    /// signature.
    #[test]
    fn signing_is_deterministic() {
        let key = SigningKey::random(&mut OsRng);
        let first: Signature = key.sign(b"message");
        let second: Signature = key.sign(b"message");
        assert_eq!(first, second);
    }

    /// The compressed SEC1 public key is 33 bytes and parses back.
    #[test]
    fn public_key_round_trips_through_sec1_bytes() -> anyhow::Result<()> {
        let key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&key);
        let encoded = verifying_key.to_encoded_point(true);
        assert_eq!(encoded.as_bytes().len(), 33);
        let parsed = VerifyingKey::from_sec1_bytes(encoded.as_bytes())?;
        assert_eq!(parsed, verifying_key);
        Ok(())
    }

    /// Truncated signature bytes are rejected.
    #[test]
    fn short_signature_is_rejected() {
        assert!(Signature::from_slice(&[0_u8; 10]).is_err());
    }
}
