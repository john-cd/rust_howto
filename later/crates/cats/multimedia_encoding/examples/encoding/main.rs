//! Runs the encoding examples.
//!
//! TODO Reviewed ??-??-2026.

mod audio_decode;
mod encoding;
mod mp4_metadata;
mod wav_encode;

/// Runs all encoding examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    encoding::run()?;
    wav_encode::run()?;
    audio_decode::run()?;
    mp4_metadata::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
