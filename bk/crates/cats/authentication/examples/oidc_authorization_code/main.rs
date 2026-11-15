//! Runs the OpenID Connect authorization-code example.
//!
//! TODO Reviewed ??-??-2026.

/// Signs a user in with the OpenID Connect authorization-code flow and PKCE,
mod oidc;

/// Runs the OpenID Connect authorization-code example.
fn main() -> anyhow::Result<()> {
    oidc::run()
}
