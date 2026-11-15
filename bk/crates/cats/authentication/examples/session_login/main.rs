//! Session login with `axum-login`, `tower-sessions` and Argon2.
//!
//! TODO Reviewed ??-??-2026.

/// Cookie-based login with `axum-login`, in-memory sessions and permissions.
mod axum_login;
/// Protect a state-changing form with a synchronizer CSRF token.
mod csrf;
/// Hash and verify passwords with Argon2id.
mod hashing;

/// # Errors
/// Returns an error if the password-hashing or Axum session-login example fails.
fn main() -> anyhow::Result<()> {
    hashing::run()?;
    csrf::run();
    axum_login::run()?;
    println!("Session login examples completed successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example end to end and asserts that it succeeds.
    #[test]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
