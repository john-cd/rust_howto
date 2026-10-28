// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Create and verify an Ed25519 signature using `ed25519-dalek`.
//!
//! The example generates a new keypair, signs a message, and verifies it.
//! Keep the signing key private; only the verifying key and signature are
//! printed.

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
}
