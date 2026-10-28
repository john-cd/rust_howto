// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates the use of the ECDSA (Elliptic Curve Digital
//! Signature Algorithm) with the P-256 curve for signing and verifying a
//! message. It uses the `p256` crate for cryptographic operations.

use p256::ecdsa::Signature;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::signature::Signer as _;
use p256::ecdsa::signature::Verifier as _;
use p256::elliptic_curve::Generate as _;
use rand::rngs::SysRng;

/// This example demonstrates the use of the ECDSA (Elliptic Curve Digital
/// Signature Algorithm) with the P-256 curve for signing and verifying a
/// message. It uses the `p256` crate for cryptographic operations.
///
/// # Errors
///
/// Returns an error if the operating system cannot provide secure randomness.
fn main() -> anyhow::Result<()> {
    // Generate a signing key:
    let mut rng = SysRng;
    let signing_key = SigningKey::try_generate_from_rng(&mut rng)?;

    // Create a verifying key from the signing key:
    let verifying_key = VerifyingKey::from(&signing_key);

    // Message to be signed:
    let message = b"Hello, world!";

    // Sign the message:
    let signature: Signature = signing_key.sign(message);

    // Verify the signature:
    assert!(verifying_key.verify(message, &signature).is_ok());
    println!("Message signature verified successfully");

    // Print the signature in hex format:
    println!("Signature: {signature:x?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the P-256 signature example.
///
/// # Errors
///
/// Returns an error if the operating system cannot provide secure randomness.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the DSA example through its entry point.
    #[test]
    fn test_dsa_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
