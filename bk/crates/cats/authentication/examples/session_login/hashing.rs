// TODO Reviewed ??-??-2026.
// TODO explain the purpose of this example and how it demonstrates password
// hashing and verification. TODO explain Argon2id and why it is used for
// password hashing.

// ANCHOR: example
//! Hash and verify passwords with Argon2id.
//!
//! This example demonstrates how to hash and verify passwords securely using
//! Argon2id. It also shows an alternative using the `password-auth` crate for
//! simpler usage.

use argon2::Argon2;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::phc::PasswordHash;

/// Hashes `password` with a random salt. The result is a self-describing PHC
/// string (algorithm, parameters, salt and hash) that is safe to store.
pub fn hash(password: &str) -> anyhow::Result<String> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|e| anyhow::anyhow!("hashing failed: {e}"))?;
    Ok(hash.to_string())
}

/// Verifies `password` against a stored PHC string. The comparison is done by
/// the library in constant time.
pub fn verify(password: &str, stored: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// The one-line alternative: the `password-auth` crate wraps Argon2 with safe
/// defaults.
pub fn hash_with_password_auth(password: &str) -> String {
    password_auth::generate_hash(password)
}
// ANCHOR_END: example

// TODO create a main() function and call it from a test and a pub fn run()
// function.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify() -> anyhow::Result<()> {
        let stored = hash("correct horse")?;
        assert!(stored.starts_with("$argon2id$"));
        assert!(verify("correct horse", &stored));
        assert!(!verify("wrong", &stored));
        assert!(!verify("anything", "not a hash"));
        Ok(())
    }

    #[test]
    fn password_auth_alternative() {
        let stored = hash_with_password_auth("pw");
        assert!(password_auth::verify_password("pw", &stored).is_ok());
    }
}
