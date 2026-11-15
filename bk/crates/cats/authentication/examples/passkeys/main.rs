//! Runs the passkey (WebAuthn) example.
//!
//! TODO Reviewed ??-??-2026.

/// A passkey (WebAuthn) relying party built on `webauthn-rs`.
mod passkeys;

/// Runs the passkey (WebAuthn) example.
fn main() -> anyhow::Result<()> {
    passkeys::run()
}

/// The entry point runs offline.
#[cfg(test)]
mod tests {
    use super::*;

    /// The entry point runs offline.
    #[test]
    fn test_main_entry_point() {
        main().expect("the software authenticator should succeed");
    }
}
