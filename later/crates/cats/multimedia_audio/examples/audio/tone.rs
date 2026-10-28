// ANCHOR: example
//! Generates a tone with `rodio` without opening an audio device.

use std::time::Duration;

use rodio::Source;
use rodio::source::SineWave;

/// Generates 100 milliseconds of a 440 Hz tone.
fn tone_samples() -> Vec<f32> {
    SineWave::new(440.0)
        .take_duration(Duration::from_millis(100))
        .collect()
}

/// Generates and checks a short tone buffer.
fn main() {
    let samples = tone_samples();
    assert!(!samples.is_empty());
    assert!(
        samples
            .iter()
            .all(|sample| sample.is_finite() && sample.abs() <= 1.0)
    );
    println!("Generated {} sine-wave samples.", samples.len());
}
// ANCHOR_END: example

/// Runs the device-free tone example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the generated tone has a bounded, finite sample buffer.
    #[test]
    fn generated_tone_contains_finite_samples() {
        let samples = tone_samples();
        assert!(!samples.is_empty());
        assert!(samples.iter().all(|sample| sample.is_finite()));
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
