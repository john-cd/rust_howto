//! Runs the audio examples.
//!
//! TODO Reviewed ??-??-2026.

mod audio;
mod midi_ports;
mod rodio_source;
mod wav_write_read;

/// Runs the audio examples.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    audio::run()?;
    wav_write_read::run()?;
    rodio_source::run();
    midi_ports::run();
    Ok(())
}
