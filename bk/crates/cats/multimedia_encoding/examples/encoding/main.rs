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
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("encoding"));
    command = command.subcommand(clap::Command::new("wav-encode"));
    command = command.subcommand(clap::Command::new("audio-decode"));
    command = command.subcommand(clap::Command::new("mp4-metadata"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("encoding", _)) => {
            encoding::run()?;
            Ok(())
        }
        Some(("wav-encode", _)) => {
            wav_encode::run()?;
            Ok(())
        }
        Some(("audio-decode", _)) => {
            audio_decode::run()?;
            Ok(())
        }
        Some(("mp4-metadata", _)) => {
            mp4_metadata::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
