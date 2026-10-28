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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
