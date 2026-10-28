//! Runs the AES-GCM and ChaCha20-Poly1305 encryption examples.

mod aes_gcm;
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
pub fn run() -> anyhow::Result<()> {
    run_examples()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies both encryption examples complete successfully.
    #[test]
    fn both_encryption_examples_succeed() -> anyhow::Result<()> {
        run_examples()
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
