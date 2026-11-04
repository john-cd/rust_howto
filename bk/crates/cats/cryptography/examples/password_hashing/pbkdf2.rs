// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of PBKDF2 for password hashing and verification.
//!
//! Servers must never store passwords directly. They store a salted,
//! deliberately slow hash instead, so a stolen database is expensive to crack.
//! The salt is a random value stored with the hash that makes equal passwords
//! hash differently. PBKDF2 slows hashing by repeating it many times; it is
//! widely supported, but Argon2 resists GPU cracking better.

// This example demonstrates how to hash and verify passwords using PBKDF2.
use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::digest;
use ring::error::Unspecified;
use ring::pbkdf2;
use ring::rand;
use ring::rand::SecureRandom as _;

/// Demonstrates the use of PBKDF2 for password hashing and verification.
fn main() -> Result<(), Unspecified> {
    /// Number of bytes in the password-hashing credential.
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).expect("the pbkdf2 example invariant should hold");
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    // Password to be hashed (in production, avoid hardcoding passwords and read
    // from environment or secrets store):
    let password = std::env::var("APP_PASSWORD")
        .unwrap_or_else(|_| "sample_password_for_demo_only".to_owned());
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    assert!(should_succeed.is_ok());

    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );
    assert!(should_fail.is_err());

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Unspecified> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the pbkdf2 example's main path succeeds.
    /// Runs the pbkdf2 example through its entry point.
    #[test]
    fn test_pbkdf2_example_entry_point() {
        assert!(main().is_ok());
    }

    /// Verifies that PBKDF2 hash verification succeeds for matching passwords
    /// and rejects incorrect attempts.
    #[test]
    fn test_pbkdf2_verification_rejects_wrong_password() {
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(10_000).expect("iteration count should be non-zero");
        let rng = rand::SystemRandom::new();
        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt)
            .expect("random salt generation should succeed");

        let password = "custom_test_password";
        let mut hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &mut hash,
        );

        let verification_success = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &hash,
        );
        assert!(verification_success.is_ok());

        let verification_failure = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            b"wrong_password",
            &hash,
        );
        assert!(verification_failure.is_err());
    }
}
