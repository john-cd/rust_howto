// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Hash and verify passwords with Argon2id.
//!
//! ### Argon2id Password Hashing
//! Argon2id is the winner of the Password Hashing Competition (PHC) and is
//! recommended by OWASP for secure password storage:
//! - **Hybrid Variant**: Argon2id combines Argon2i (resistant to side-channel
//!   attacks) and Argon2d (resistant to GPU cracking attacks).
//! - **Memory-Hard Algorithm**: Configurable memory, time cost, and parallelism
//!   parameters significantly increase the resource cost for attackers
//!   attempting brute-force dictionary attacks.
//! - **PHC String Standard**: Output hashes are formatted as standard
//!   self-describing PHC strings `$argon2id$v=...$m=...,t=...,p=...$salt$hash`
//!   containing algorithm parameters, salt, and hash.
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

/// Hashes and verifies a sample password.
fn main() -> anyhow::Result<()> {
    let password = "correct horse battery staple";
    let stored_hash = hash(password)?;
    assert!(verify(password, &stored_hash));
    println!("Successfully hashed and verified password with Argon2id.");

    // The one-line alternative:
    let stored_hash2 = hash_with_password_auth(password);
    assert!(password_auth::verify_password(password, &stored_hash2).is_ok());
    Ok(())
}

/// Runs the hashing example.
pub fn run() -> anyhow::Result<()> {
    main()
}

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

    #[test]
    fn test_hashing_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
