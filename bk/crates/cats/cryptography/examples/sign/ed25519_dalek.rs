// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates signing and verifying messages with ed25519-dalek.
//!
//! A digital signature proves who wrote a message and that it was not altered:
//! the signer uses a private key and anyone can check the result with the
//! public key. Ed25519 is a fast modern scheme used for SSH keys and signed
//! software updates.

use ed25519_dalek::Signature;
use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier as _;
use ed25519_dalek::VerifyingKey;
use rand::rngs::SysRng;
use rand_core::UnwrapErr;

/// Ed25519 Signature Example
///
/// This example demonstrates the basic usage of the `ed25519-dalek` crate for
/// generating Ed25519 keypairs, signing messages, and verifying signatures.
fn main() -> anyhow::Result<()> {
    // First, we need to generate a Keypair, which includes both public and
    // secret halves of an asymmetric key. To do so, we need a
    // cryptographically secure pseudorandom number generator (CSPRNG).
    // For this example, we'll use the operating system's builtin PRNG:

    let mut csprng = UnwrapErr(SysRng);
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    // Message to be signed:
    let message = b"Hello, world!";

    // Sign the message:
    let signature: Signature = signing_key.sign(message);

    // Anyone else, given the public half of the signing_key can also easily
    // verify this signature:
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    verifying_key.verify(message, &signature)?;
    anyhow::ensure!(
        verifying_key
            .verify(b"Changed message", &signature)
            .is_err(),
        "a signature must not verify for a different message"
    );

    // VerifyingKeys, SecretKeys, SigningKeys, and Signatures can be
    // serialised into byte-arrays by calling `to_bytes`.

    println!("Ed25519 signature verified successfully!");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies Ed25519 signatures reject a changed message.
    /// Runs the ed25519 dalek example through its entry point.
    #[test]
    fn test_ed25519_dalek_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn tampered_signature_is_rejected() {
        let signing_key = SigningKey::from_bytes(&[9u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let signature: Signature = signing_key.sign(b"Hello, world!");
        assert!(verifying_key.verify(b"Hello, world!", &signature).is_ok());

        let mut bytes = signature.to_bytes();
        bytes[0] ^= 0xff;
        let tampered = Signature::from_bytes(&bytes);
        assert!(verifying_key.verify(b"Hello, world!", &tampered).is_err());
    }

    #[test]
    fn verifying_key_roundtrips_through_bytes() -> anyhow::Result<()> {
        let key = SigningKey::from_bytes(&[3u8; 32]).verifying_key();
        assert_eq!(VerifyingKey::from_bytes(&key.to_bytes())?, key);
        Ok(())
    }
}
