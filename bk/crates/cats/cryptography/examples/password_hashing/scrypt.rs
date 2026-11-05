//! TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Hashing and verifying a password with `scrypt`.
//!
//! Servers must never store passwords directly; they store a salted,
//! deliberately slow hash. scrypt is memory-hard, which makes cracking with
//! specialized hardware expensive. The salt is random data stored with the hash
//! so equal passwords do not hash alike.

// This example demonstrates how to hash and verify passwords using scrypt.
use std::error::Error;

use scrypt::Scrypt;
use scrypt::password_hash::PasswordHasher as _;
use scrypt::password_hash::PasswordVerifier as _;
use scrypt::password_hash::phc::PasswordHash;

/// This example demonstrates how to use the `scrypt` crate to hash and verify
/// passwords.
fn example() -> Result<(), Box<dyn Error>> {
    // 1) When setting the password, hash it and store the hash.

    // Password to be hashed (in production, avoid hardcoding passwords and read
    // from environment variables or a secrets store):
    let password = std::env::var("APP_PASSWORD")
        .unwrap_or_else(|_| "sample_password_for_demo_only".to_owned());
    let password = password.as_bytes();

    let scrypt = Scrypt::default();

    // Hash the password to a PHC string ($scrypt$...) with a generated salt:
    let password_hash: PasswordHash = scrypt.hash_password(password)?;

    // Print the hashed password:
    let password_hash = password_hash.to_string();
    println!("Hashed password: {password_hash}");

    // 2) Later, in order to verify a password, the hash is retrieved from the
    // database, and the password is checked against it.

    let parsed_hash = PasswordHash::new(&password_hash)?;
    let is_valid = scrypt.verify_password(password, &parsed_hash).is_ok();

    // Print the verification result:
    println!("Password is valid: {is_valid}");

    Ok(())
}

/// Runs the scrypt example.
fn main() -> Result<(), Box<dyn Error>> {
    example()
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scrypt example through its entry point.
    #[test]
    fn test_scrypt_example_entry_point() {
        main().expect("example entry point should succeed");
    }

    /// Verifies that hashing and verification works with a custom password.
    #[test]
    fn test_scrypt_hash_and_verify() -> Result<(), Box<dyn Error>> {
        let password = b"custom_secret_passphrase";
        let scrypt = Scrypt::default();
        let password_hash = scrypt.hash_password(password)?.to_string();
        let parsed_hash = PasswordHash::new(&password_hash)?;

        assert!(scrypt.verify_password(password, &parsed_hash).is_ok());
        assert!(
            scrypt
                .verify_password(b"wrong_guess", &parsed_hash)
                .is_err()
        );
        Ok(())
    }

    /// Verifies that hashing the same password twice yields distinct hashes due to random salting.
    #[test]
    fn test_scrypt_random_salt() -> Result<(), Box<dyn Error>> {
        let password = b"identical_password";
        let scrypt = Scrypt::default();
        let hash1 = scrypt.hash_password(password)?.to_string();
        let hash2 = scrypt.hash_password(password)?.to_string();

        assert_ne!(hash1, hash2);
        Ok(())
    }
}
