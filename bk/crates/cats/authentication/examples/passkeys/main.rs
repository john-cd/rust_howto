//! Runs the passkey (WebAuthn) example.
//!
//! TODO Reviewed ??-??-2026.

/// A passkey (WebAuthn) relying party built on `webauthn-rs`.
mod passkeys;

/// Runs the passkey (WebAuthn) example.
fn main() -> anyhow::Result<()> {
    passkeys::run()
}
