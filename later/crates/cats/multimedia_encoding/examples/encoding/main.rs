//! Runs the encoding examples.
//!
//! TODO Reviewed ??-??-2026.

/// Probe and decode audio formats to linear PCM samples using `symphonia`.
mod audio_decode;
/// Encode a blank AV1 frame using the `rav1e` crate.
mod encoding;
/// Inspect MP4 container metadata including tracks and duration using `mp4`.
mod mp4_metadata;
/// Encode PCM audio samples to a WAV buffer using `hound`.
mod wav_encode;

/// Runs all encoding examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    encoding::run()?;
    wav_encode::run()?;
    audio_decode::run()?;
    mp4_metadata::run()?;
    Ok(())
}
