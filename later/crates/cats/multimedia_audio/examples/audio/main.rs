//! Runs the audio example.
//!
//! TODO Reviewed ??-??-2026

mod audio;

/// Runs the example from the `audio` module.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    audio::run()?;
    Ok(())
}
