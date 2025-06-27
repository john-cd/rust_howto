#![allow(dead_code)]
// ANCHOR: example
use std::error::Error;

use scrypt::Scrypt;
use scrypt::password_hash::PasswordHash;
use scrypt::password_hash::PasswordHasher;
use scrypt::password_hash::PasswordVerifier;
use scrypt::password_hash::SaltString;
use scrypt::password_hash::rand_core::OsRng;

/// This example demonstrates how to use the `scrypt` crate to hash and verify
/// passwords.
fn main() -> Result<(), Box<dyn Error>> {
    // 1) When setting the password, hash it and store the hash

    // Password to be hashed
    let password = b"super_secret_password";

    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash the password to a PHC string ($scrypt$...)
    let password_hash: PasswordHash<'_> =
        Scrypt.hash_password(password, &salt)?;

    // Print the hashed password
    let password_hash = password_hash.to_string();
    println!("Hashed password: {password_hash}");

    // 2) Later, in order to verify a password, the hash is retrieved from the
    // database, and the password is checked against it.

    let parsed_hash = PasswordHash::new(&password_hash)?;
    let is_valid = Scrypt.verify_password(password, &parsed_hash).is_ok();

    // Print the verification result
    println!("Password is valid: {is_valid}");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
