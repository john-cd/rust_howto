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
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("audio"));
    command = command.subcommand(clap::Command::new("hrtf"));
    command = command.subcommand(clap::Command::new("wav-write-read"));
    command = command.subcommand(clap::Command::new("rodio-file"));
    command = command.subcommand(clap::Command::new("tone"));
    command = command.subcommand(clap::Command::new("resampling"));
    command = command.subcommand(clap::Command::new("rodio-source"));
    command = command.subcommand(clap::Command::new("midi-ports"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("audio", _)) => {
            audio::run()?;
            Ok(())
        }
        Some(("hrtf", _)) => {
            hrtf::run()?;
            Ok(())
        }
        Some(("wav-write-read", _)) => {
            wav_write_read::run()?;
            Ok(())
        }
        Some(("rodio-file", _)) => {
            rodio_file::run()?;
            Ok(())
        }
        Some(("tone", _)) => {
            tone::run();
            Ok(())
        }
        Some(("resampling", _)) => {
            resampling::run()?;
            Ok(())
        }
        Some(("rodio-source", _)) => {
            rodio_source::run();
            Ok(())
        }
        Some(("midi-ports", _)) => {
            midi_ports::run();
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
