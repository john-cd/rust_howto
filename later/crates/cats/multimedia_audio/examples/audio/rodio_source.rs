#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Plays a sound produced by a custom `rodio::Source`.
//!
//! A source is an iterator of samples, pulled by the audio thread.

use std::error::Error;
use std::f32::consts::TAU;
use std::time::Duration;

use rodio::ChannelCount;
use rodio::DeviceSinkBuilder;
use rodio::Player;
use rodio::SampleRate;
use rodio::Source;
use rodio::nz;

/// A mono sine wave.
struct Sine {
    /// Frequency in hertz. Whole numbers loop cleanly every second.
    frequency: f32,
    /// Samples per second.
    sample_rate: SampleRate,
    /// Index of the next sample within the current second.
    position: u32,
}

impl Sine {
    /// Creates a sine wave at the CD sample rate.
    fn new(frequency: f32) -> Self {
        Self {
            frequency,
            sample_rate: nz!(44_100),
            position: 0,
        }
    }
}

impl Iterator for Sine {
    type Item = f32;

    /// Computes the next sample. The audio thread calls this function
    /// repeatedly: it must be fast and must not block or allocate.
    fn next(&mut self) -> Option<f32> {
        let rate = self.sample_rate.get();
        let seconds = self.position as f32 / rate as f32;
        // Wrapping the position keeps the phase accurate for a long time.
        self.position = (self.position + 1) % rate;
        Some((TAU * self.frequency * seconds).sin())
    }
}

impl Source for Sine {
    /// `None`: the format never changes.
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        nz!(1)
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// `None`: the sound never ends.
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Plays a chord for one second. It needs an audio device.
fn play_chord() -> Result<(), Box<dyn Error>> {
    // The sound stops when the device sink is dropped.
    let sink = DeviceSinkBuilder::open_default_sink()?;
    let player = Player::connect_new(sink.mixer());
    let chord = Sine::new(440.0)
        .mix(Sine::new(660.0))
        .amplify(0.2)
        .take_duration(Duration::from_secs(1));
    player.append(chord);
    // Wait: the playback happens on another thread.
    player.sleep_until_end();
    Ok(())
}

/// Pulls samples from the source, as the audio thread would, without a device.
fn main() {
    let mut sine = Sine::new(440.0);
    let samples: Vec<f32> = sine.by_ref().take(44_100).collect();

    assert_eq!(samples[0], 0.0);
    assert!(samples.iter().all(|s| s.abs() <= 1.0));

    // 440 rising zero crossings in one second: one per period.
    let crossings = samples.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0);
    assert!((439..=441).contains(&crossings.count()));
    println!("A4 generated; call play_chord() to hear it.");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn source_has_a_fixed_format() {
        let sine = Sine::new(440.0);
        assert_eq!(sine.channels().get(), 1);
        assert_eq!(sine.sample_rate().get(), 44_100);
        assert_eq!(sine.total_duration(), None);
    }
}
