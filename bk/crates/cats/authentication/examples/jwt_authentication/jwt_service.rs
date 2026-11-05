// TODO Reviewed ??-??-2026.

//! Demonstrates creating, signing, and verifying a JSON Web Token (JWT) as well
//! as configuring an Axum router protected by bearer-token authentication.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context as _;

use super::jwt;
use super::jwt_axum_extractor;

/// Deterministic test secret key of sufficient length for HS256 HMAC operations.
pub const TEST_JWT_SECRET: &str = "default_test_secret_key_that_is_at_least_32_bytes_long";

/// Signs and verifies an in-memory token using the `JWT_SECRET` environment
/// variable.
///
/// # Errors
/// Returns an error if `JWT_SECRET` is missing, too short, or token creation
/// or verification fails.
pub fn run_from_env() -> anyhow::Result<()> {
    let secret =
        std::env::var("JWT_SECRET").context("set JWT_SECRET to at least 32 random bytes")?;
    run_with_secret(&secret)
}

/// Signs and verifies a short-lived token with `secret`.
///
/// # Errors
/// Returns an error if the secret is invalid or token creation or verification
/// fails.
pub fn run_with_secret(secret: &str) -> anyhow::Result<()> {
    let jwt_secret = jwt_axum_extractor::JwtSecret::new(secret.as_bytes())?;
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_secs()
        + 60;
    let claims = jwt::Claims {
        sub: "example-user".to_owned(),
        exp,
        roles: vec!["reader".to_owned()],
    };
    let token = jwt::create_jwt(&claims, secret.as_bytes())?;
    let verified = jwt::verify_jwt(&token, secret.as_bytes())?;
    let _admin_router = jwt_axum_extractor::admin_router(jwt_secret);

    println!(
        "Created and verified a short-lived token for {}.",
        verified.sub
    );
    Ok(())
}

/// Entry point for running the JWT authentication example with the default test secret.
fn main() -> anyhow::Result<()> {
    run_with_secret(TEST_JWT_SECRET)
}

/// Runs the example with a deterministic test secret without changing the
/// process environment.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example succeeds with its injected test secret and fails on invalid
    /// secrets.
    #[test]
    fn test_jwt_service_entry_point() -> anyhow::Result<()> {
        main()?;
        assert!(run_with_secret("too-short").is_err());
        Ok(())
    }
}
