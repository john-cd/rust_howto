// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Play a sine tone on the default audio output with `rodio`.
//!
//! The tone is built first, so its properties can be checked without an audio
//! device; playing it opens the default output device.

use std::error::Error;
use std::time::Duration;

use rodio::DeviceSinkBuilder;
use rodio::Player;
use rodio::Source;
use rodio::source::SineWave;

/// The pitch of the tone, in hertz (concert A).
const FREQUENCY_HZ: f32 = 440.0;

/// How long the tone plays.
const DURATION: Duration = Duration::from_secs(2);

/// Builds a sine tone that stops after [`DURATION`].
fn tone() -> impl Source {
    SineWave::new(FREQUENCY_HZ).take_duration(DURATION)
}

/// Plays a 440 Hz sine tone for two seconds on the default output device.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Keep the sink alive: dropping it stops the audio.
    let sink = DeviceSinkBuilder::open_default_sink()?;
    let player = Player::connect_new(sink.mixer());

    player.append(tone());
    // Block until the tone has finished.
    player.sleep_until_end();

    println!("Played a 440 Hz sine tone for 2 seconds.");
    Ok(())
}
// ANCHOR_END: example

/// Plays the tone.
///
/// # Errors
///
/// Returns an error if no audio output device is available.
pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    /// The tone is mono and stops after about two seconds of samples, so it
    /// needs no device.
    #[test]
    fn tone_has_the_expected_format() {
        let source = tone();
        let channels = NonZeroUsize::from(source.channels()).get();
        let sample_rate = usize::try_from(source.sample_rate().get())
            .expect("the sample rate fits in usize");
        assert_eq!(channels, 1);
        let samples = source.count();
        let expected = sample_rate * channels * 2;
        assert!(
            samples.abs_diff(expected) <= expected / 100,
            "{samples} samples, expected about {expected}"
        );
    }

    /// Opening the default sink succeeds when an output device exists.
    #[test]
    #[ignore = "needs an audio output device"]
    fn default_sink_can_be_opened() -> Result<(), Box<dyn Error + Send + Sync>> {
        let _sink = DeviceSinkBuilder::open_default_sink()?;
        Ok(())
    }

    /// Plays the tone through the entry point.
    #[test]
    #[ignore = "needs an audio output device"]
    fn test_play_sound_example_entry_point()
    -> Result<(), Box<dyn Error + Send + Sync>> {
        run()
    }
}
