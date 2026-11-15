//! Runs the audio examples.
//!
//! TODO Reviewed ??-??-2026.

/// A simple example that plays a 440Hz sine wave (A4) for 1 second.
mod audio;
/// Renders a mono tone as binaural stereo with `hrtf`.
mod hrtf;
/// Lists MIDI input ports with `midir` and decodes raw MIDI messages.
mod midi_ports;
/// Resamples a mono buffer from 48 kHz to 44.1 kHz with `rubato`.
mod resampling;
/// Decodes a WAV in memory and shows how to play a file with `rodio`.
mod rodio_file;
/// Plays a sound produced by a custom `rodio::Source`.
mod rodio_source;
/// Generates a tone with `rodio` without opening an audio device.
mod tone;
/// Writes one second of a 440 Hz sine wave to an in-memory WAV file with
mod wav_write_read;

/// Runs the audio examples.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    audio::run()?;
    hrtf::run()?;
    wav_write_read::run()?;
    rodio_file::run()?;
    tone::run();
    resampling::run()?;
    rodio_source::run();
    midi_ports::run();
    Ok(())
}
