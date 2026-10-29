//! Runs the audio examples.
//!
//! TODO Reviewed ??-??-2026.

mod audio;
mod hrtf;
mod midi_ports;
mod resampling;
mod rodio_file;
mod rodio_source;
mod tone;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[ignore = "requires an audio output device"]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
