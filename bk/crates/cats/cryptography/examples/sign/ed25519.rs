// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
//! Create and verify an Ed25519 signature using `ed25519-dalek`.
//!
//! The example generates a new keypair, signs a message, and verifies it.

use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier;
use rand_core::OsRng;

fn main() -> anyhow::Result<()> {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    let message = b"This is a message to be signed.";

    let signature: ed25519::Signature = signing_key.sign(message);
    verifying_key.verify(message, &signature)?;

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
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/1084)
}
