//! Session login with `axum-login`, `tower-sessions` and Argon2.
//!
//! TODO Reviewed ??-??-2026.

mod axum_login;
mod csrf;
mod hashing;

fn main() -> anyhow::Result<()> {
    hashing::run()?;
    csrf::run();
    axum_login::run()?;
    println!("Session login examples completed successfully.");
    Ok(())
}
