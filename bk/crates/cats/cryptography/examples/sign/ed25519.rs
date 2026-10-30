// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Create and verify an Ed25519 signature using `ed25519-dalek`.
//!
//! The example generates a new keypair, signs a message, and verifies it.
//! Keep the signing key private; only the verifying key and signature are
//! printed.
//!
//! A digital signature proves who wrote a message and that it was not altered.
//! The signer keeps a private signing key; anyone with the matching public
//! verifying key can check the signature, but cannot forge one. Ed25519 is a
//! fast modern signature scheme used for SSH keys, software updates and signed
//! tokens.

use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier as _;
use rand::rngs::SysRng;
use rand_core::UnwrapErr;

/// Signs a message with a random Ed25519 key and verifies the signature.
fn main() -> anyhow::Result<()> {
    let mut rng = UnwrapErr(SysRng);
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    let message = b"This is a message to be signed.";

    let signature: ed25519::Signature = signing_key.sign(message);
    verifying_key.verify(message, &signature)?;
    anyhow::ensure!(
        verifying_key
            .verify(b"This message was changed.", &signature)
            .is_err(),
        "the signature must not verify for a different message"
    );

    println!("Verifying key: {:?}", verifying_key.to_bytes());
    println!("Signature bytes: {:?}", signature.to_bytes());
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Signs and verifies the original message while rejecting a changed one.
    /// Runs the ed25519 example through its entry point.
    #[test]
    fn test_ed25519_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn signature_verifies_only_for_the_signed_message() {
        let signing_key = SigningKey::from_bytes(&[7u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let signature: ed25519::Signature = signing_key.sign(b"msg");
        assert!(verifying_key.verify(b"msg", &signature).is_ok());
        assert!(verifying_key.verify(b"other", &signature).is_err());
    }

    #[test]
    fn signing_is_deterministic_and_keys_are_distinct() {
        let a = SigningKey::from_bytes(&[1u8; 32]);
        let b = SigningKey::from_bytes(&[2u8; 32]);
        assert_eq!(a.sign(b"m").to_bytes(), a.sign(b"m").to_bytes());
        assert_ne!(a.sign(b"m").to_bytes(), b.sign(b"m").to_bytes());
        let sig = a.sign(b"m");
        assert!(b.verifying_key().verify(b"m", &sig).is_err());
        assert_eq!(sig.to_bytes().len(), 64);
        assert_eq!(a.verifying_key().to_bytes().len(), 32);
    }
}
