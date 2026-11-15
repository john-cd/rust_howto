//! Runs the AES-GCM and ChaCha20-Poly1305 encryption examples.
//!
//! TODO Reviewed ??-??-2026.

/// Authenticated encryption and decryption with AES-256-GCM.
mod aes_gcm;
/// Authenticated encryption with ChaCha20-Poly1305, with and without associated
mod chacha20poly1305;

/// Runs both authenticated-encryption examples.
fn main() -> anyhow::Result<()> {
    run_examples()
}
/// Runs each authenticated-encryption recipe in sequence.

fn run_examples() -> anyhow::Result<()> {
    aes_gcm::run()?;
    chacha20poly1305::run()?;
    Ok(())
}

/// Runs the encryption examples.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    run_examples()
}

/// Verifies that the AES-GCM and ChaCha20-Poly1305 encryption examples.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_encryption_examples_succeed() {
        run_examples().expect("both authenticated-encryption examples succeed");
    }
}
