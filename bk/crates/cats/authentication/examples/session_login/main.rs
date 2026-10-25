//! Session login with `axum-login`, `tower-sessions` and Argon2.
//!
//! TODO Reviewed ??-??-2026.

mod axum_login;
mod csrf;
mod hashing;

fn main() {
    println!("Run `cargo test --example session_login`.");
}
