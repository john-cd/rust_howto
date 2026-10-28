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
