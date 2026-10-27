#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `bcrypt` crate to hash and verify
//! passwords.
//!
//! The `bcrypt` crate provides a way to securely hash passwords using the
//! bcrypt algorithm. It is a one-way function, meaning that it is
//! computationally infeasible to reverse the hash.

use std::error::Error;

use bcrypt::DEFAULT_COST;
use bcrypt::hash;
use bcrypt::verify;

/// Hashes a password with `bcrypt`, then verifies it.
fn main() -> Result<(), Box<dyn Error>> {
    // 1) When setting the password, hash it and store the hash:

    // Password to be hashed (in production, avoid hardcoding passwords and read
    // from environment or secrets store):
    let password = std::env::var("APP_PASSWORD")
        .unwrap_or_else(|_| "sample_password_for_demo_only".to_owned());

    // Generate a password hash using the default cost.
    // The salt is generated randomly using OS-derived randomness:
    let hashed_password: String = hash(&password, DEFAULT_COST)?;

    println!("Hashed password: {hashed_password}");

    // 2) Later, in order to verify a password, the hash is retrieved from the
    // database, and the password is checked against it:

    let is_valid = verify(&password, &hashed_password)?;
    println!("Password is valid: {is_valid}");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the bcrypt example through its entry point.
    #[test]
    fn test_bcrypt_example_entry_point() -> Result<(), Box<dyn Error>> {
        main()?;
        Ok(())
    }
}
