//! Signs and verifies a JWT and demonstrates an Axum bearer-token extractor.
//!
//! TODO Reviewed ??-??-2026.

/// Encode and verify HS256 JSON Web Tokens for a first-party API.
mod jwt;

/// Extract a JWT from the HTTP `Authorization` header and check a role in Axum.
mod jwt_axum_extractor;

/// Demonstrates creating, signing, and verifying a JSON Web Token (JWT) as well as configuring an
/// Axum router protected by bearer-token authentication.
mod jwt_service;

/// Signs and verifies an in-memory token using the `JWT_SECRET` environment
///
/// variable if set, otherwise falling back to the test secret for demonstration.
///
/// # Errors
///
/// Returns an error if token creation or verification fails.
fn main() -> anyhow::Result<()> {
    let _claims = jwt::run()?;
    let _router = jwt_axum_extractor::run()?;

    if std::env::var("JWT_SECRET").is_ok() {
        jwt_service::run_from_env()
    } else {
        jwt_service::run()
    }
}
