#![allow(dead_code)]
// ANCHOR: example
// An error with absolutely no details (on purpose):
use ring::error::Unspecified;
use ring::hmac;
use ring::rand;
use ring::rand::SecureRandom;

/// This example demonstrates how to use the `ring` crate to generate an HMAC
/// (Hash-based Message Authentication Code) signature for a message and then
/// verify the signature. It uses SHA256 as the underlying hash function.
fn main() -> Result<(), Unspecified> {
    // 1. Create a key:
    let key;
    {
        let mut key_value = [0u8; 48];
        let rng = rand::SystemRandom::new();
        rng.fill(&mut key_value)?;
        // Construct an HMAC signing key using the given digest algorithm
        // and key value. `key_value`` should be a value generated with
        // a secure random number generator.
        key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    }

    // 2. Sign a message:
    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());

    // 3. Calculates the HMAC of data using the signing key,
    // and verifies whether the resultant value equals the signature.
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    println!("Message verified.");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main().map_err(|_| anyhow::anyhow!("hmac.rs failure"))?;
    Ok(())
}
