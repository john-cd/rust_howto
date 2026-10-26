#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Examples of password hashing and key derivation with Argon2.

use argon2::Argon2;
use argon2::password_hash::Error;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher as _;
use argon2::password_hash::PasswordVerifier as _;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

/// Hashes a password with Argon2, then verifies it and derives a key.
fn main() -> Result<(), Error> {
    let password_to_hash = b"super_secret_password"; // Bad password; don't actually use!

    let phc_string = password_hashing(password_to_hash)?;
    println!("Hashed password: {phc_string}");

    // In a real application, save the PHC string to a secure database or file.
    // The PHC string contains the (one-way) hashed password and the salt, not
    // the password. Should an attacker gain access to the database or file,
    // they won't be able to retrieve the password.

    // When a user logs in, the password they provide is checked against the
    // saved PHC string: The correct password passes the verification
    // function.
    let password_to_check = password_to_hash;
    let is_valid = password_verification(password_to_check, &phc_string)?;
    println!("The password is valid: {is_valid}");

    // An incorrect password fails the check:
    assert!(!password_verification(b"random_guess", &phc_string)?);

    // Key derivation: derive a child key from a password and salt:
    key_derivation()?;

    Ok(())
}

/// Hash a password to a "PHC string" suitable for the purposes of
/// password-based authentication.
fn password_hashing(password_to_save: &[u8]) -> Result<String, Error> {
    // Generate a random salt:
    let salt = SaltString::generate(&mut OsRng);

    // Configure `Argon2` with default params (Argon2id v19):
    let argon2 = Argon2::default();

    // Hash the password to a PHC string ($argon2id$v=19$...):
    let phc_string = argon2.hash_password(password_to_save, &salt)?.to_string();

    Ok(phc_string)
}

/// Verify a given password against the provided PHC string.
/// Returns true if the password is correct.
fn password_verification(
    password_to_check: &[u8],
    phc_string: &str,
) -> Result<bool, Error> {
    password_verification_with(
        &Argon2::default(),
        password_to_check,
        phc_string,
    )
}

/// Verifies a password using the provided Argon2 configuration.
fn password_verification_with(
    argon2: &Argon2<'_>,
    password_to_check: &[u8],
    phc_string: &str,
) -> Result<bool, Error> {
    // Parsed representation of a PHC string:
    let parsed_hash = PasswordHash::new(phc_string)?;

    // Verify the password:
    Ok(argon2
        .verify_password(password_to_check, &parsed_hash)
        .is_ok())
}

/// Transform a password into cryptographic keys that can be used for e.g.
/// encryption.
fn key_derivation() -> Result<(), Error> {
    let password = b"passwd"; // Bad password; don't use!
    let salt = b"example salt, etc, etc"; // Salt should be unique per password

    let mut output_key_material = [0u8; 128]; // Can be any desired size

    // Hash a password and associated parameters into the provided output
    // buffer.
    Argon2::default().hash_password_into(
        password,
        salt,
        &mut output_key_material,
    )?;

    // `output_key_material` can now be used as a cryptographic key.

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use argon2::Algorithm;
    use argon2::Params;
    use argon2::Version;

    use super::*;

    /// Verifies a password hash accepts the original password and rejects a
    /// guess.
    #[test]
    fn argon2_verification_rejects_wrong_password() -> Result<(), Error> {
        let params = Params::new(32, 1, 1, None)?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let salt = SaltString::generate(&mut OsRng);
        let phc_string = argon2
            .hash_password(b"correct horse battery staple", &salt)?
            .to_string();

        assert!(password_verification_with(
            &argon2,
            b"correct horse battery staple",
            &phc_string
        )?);
        assert!(!password_verification_with(
            &argon2,
            b"incorrect",
            &phc_string
        )?);
        Ok(())
    }

    /// Reports malformed PHC strings as errors instead of accepting them.
    #[test]
    fn argon2_verification_rejects_malformed_hash() {
        assert!(
            password_verification(b"password", "not a PHC string").is_err()
        );
    }
}
