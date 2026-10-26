#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates signing and verifying messages with ed25519-dalek.

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
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
