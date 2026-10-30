// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `ring` crate to generate an HMAC
//! (Hash-based Message Authentication Code) signature for a message and then
//! verify the signature. It uses SHA256 as the underlying hash function.

// An error with absolutely no details (on purpose):
use ring::error::Unspecified;
use ring::hmac;
use ring::rand;
use ring::rand::SecureRandom as _;

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

pub fn run() -> Result<(), Unspecified> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the hmac example's main path succeeds.
    /// Runs the hmac example through its entry point.
    #[test]
    fn test_hmac_example_entry_point() -> anyhow::Result<()> {
        main().map_err(|_| anyhow::anyhow!("hmac.rs failure"))?;
        Ok(())
    }

    #[test]
    fn verification_fails_for_other_messages_and_keys() {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &[1u8; 48]);
        let tag = hmac::sign(&key, b"message");
        assert_eq!(tag.as_ref().len(), 32);
        assert!(hmac::verify(&key, b"message", tag.as_ref()).is_ok());
        assert!(hmac::verify(&key, b"messagE", tag.as_ref()).is_err());
        let other = hmac::Key::new(hmac::HMAC_SHA256, &[2u8; 48]);
        assert!(hmac::verify(&other, b"message", tag.as_ref()).is_err());
    }

    #[test]
    fn matches_the_rfc_4231_test_vector() {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &[0x0b; 20]);
        let tag = hmac::sign(&key, b"Hi There");
        assert_eq!(
            tag.as_ref()[..4],
            [0xb0, 0x34, 0x4c, 0x61],
            "RFC 4231 case 1 starts with b0344c61"
        );
    }
}
