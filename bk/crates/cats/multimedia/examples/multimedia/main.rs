//! Runs the multimedia example.
//!
//! TODO Reviewed ??-??-2026.

/// Play a sine tone on the default audio output with `rodio`.
mod play_sound;

/// Runs the multimedia example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    play_sound::run()
}
