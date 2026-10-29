// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Hashing and verifying a password with `scrypt`.

// This example demonstrates how to hash and verify passwords using scrypt.
use std::error::Error;

use scrypt::Scrypt;
use scrypt::password_hash::PasswordHasher as _;
use scrypt::password_hash::PasswordVerifier as _;
use scrypt::password_hash::phc::PasswordHash;

/// This example demonstrates how to use the `scrypt` crate to hash and verify
/// passwords.
fn example() -> Result<(), Box<dyn Error>> {
    // 1) When setting the password, hash it and store the hash

    // Password to be hashed:
    let password = b"super_secret_password";

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
    example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scrypt example through its entry point.
    #[test]
    fn test_scrypt_example_entry_point() {
        main().expect("example entry point should succeed");
    }
}
