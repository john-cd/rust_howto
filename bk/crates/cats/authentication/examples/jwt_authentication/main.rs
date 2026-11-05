//! Signs and verifies a JWT and demonstrates an Axum bearer-token extractor.
//!
//! TODO Reviewed ??-??-2026.

mod jwt;
mod jwt_axum_extractor;
mod jwt_service;

/// Signs and verifies an in-memory token using the `JWT_SECRET` environment
/// variable if set, otherwise falling back to the test secret for demonstration.
///
/// # Errors
/// Returns an error if token creation or verification fails.
fn main() -> anyhow::Result<()> {
    if std::env::var("JWT_SECRET").is_ok() {
        jwt_service::run_from_env()
    } else {
        jwt_service::run()
    }
}

/// Runs the example with a deterministic test secret without changing the
/// process environment.
pub fn run() -> anyhow::Result<()> {
    jwt_service::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example succeeds through its main entry point.
    #[test]
    fn test_jwt_authentication_entry_point() {
        assert!(main().is_ok());
    }
}
