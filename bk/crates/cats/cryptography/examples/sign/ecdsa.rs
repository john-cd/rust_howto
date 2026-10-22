#![allow(dead_code)]

// ANCHOR: example
//! Sign and verify a message with ECDSA using the P-256 curve.
//!
//! This example uses `p256` and the `Signer`/`Verifier` traits.
//!
//! TODO Reviewed ??-??-2026

use p256::ecdsa::Signature;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::signature::Signer as _;
use p256::ecdsa::signature::Verifier as _;
use p256::elliptic_curve::rand_core::OsRng;

fn main() -> anyhow::Result<()> {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let message = b"Hello, world!";

    let signature: Signature = signing_key.sign(message);
    verifying_key.verify(message, &signature)?;

    println!(
        "Compressed public key bytes: {:?}",
        verifying_key.to_encoded_point(true).as_bytes()
    );
    println!("Signature: {signature:?}");
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
    // [finish](https://github.com/john-cd/rust_howto/issues/693)
}
