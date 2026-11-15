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
