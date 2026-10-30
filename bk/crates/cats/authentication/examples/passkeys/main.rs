//! Runs the passkey (WebAuthn) example.
//!
//! TODO Reviewed ??-??-2026.

mod passkeys;

/// Runs the passkey (WebAuthn) example.
fn main() -> anyhow::Result<()> {
    passkeys::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The entry point runs offline.
    #[test]
    fn test_main_entry_point() {
        main().expect("the software authenticator should succeed");
    }
}
