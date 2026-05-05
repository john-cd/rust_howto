#![allow(dead_code)]
// ANCHOR: example
// This example demonstrates how to hash and verify passwords using scrypt.
use std::error::Error;

use scrypt::Scrypt;
use scrypt::password_hash::PasswordHasher;
use scrypt::password_hash::PasswordVerifier;
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
    #[test]
    fn test_main() {
        main().unwrap();
    }
}
