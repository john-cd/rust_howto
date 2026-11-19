// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Finds the dominant frequency in a synthetic sampled signal.

use std::f32::consts::TAU;

use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

const SAMPLE_RATE_HZ: f32 = 128.0;
const SAMPLE_COUNT: usize = 256;
const SIGNAL_HZ: f32 = 10.0;

/// Applies a Hann window, computes the forward FFT, and finds its peak bin.
fn dominant_frequency_hz() -> f32 {
    let mut samples: Vec<_> = (0..SAMPLE_COUNT)
        .map(|index| {
            let sample_index = index as f32;
            let phase = TAU * SIGNAL_HZ * sample_index / SAMPLE_RATE_HZ;
            let window_phase = TAU * sample_index / (SAMPLE_COUNT - 1) as f32;
            let window = 0.5 * (1.0 - window_phase.cos());
            Complex::new(phase.sin() * window, 0.0)
        })
        .collect();

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(SAMPLE_COUNT);
    fft.process(&mut samples);

    let peak_bin = (1..SAMPLE_COUNT / 2)
        .max_by(|left, right| samples[*left].norm().total_cmp(&samples[*right].norm()))
        .expect("the positive-frequency range contains bins");
    peak_bin as f32 * SAMPLE_RATE_HZ / SAMPLE_COUNT as f32
}

/// Reports the estimated frequency of a known synthetic signal.
fn main() {
    let estimated_hz = dominant_frequency_hz();
    assert!((estimated_hz - SIGNAL_HZ).abs() < 0.25);
    println!("Estimated frequency: {estimated_hz:.1} Hz");
}
// ANCHOR_END: example

/// Runs the synthetic signal analysis example.
pub fn run() {
    main();
}

/// Checks that the FFT locates the known sinusoid's frequency bin.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the FFT locates the known sinusoid's frequency bin.
    #[test]
    fn finds_the_synthetic_signal_frequency() {
        assert!((dominant_frequency_hz() - SIGNAL_HZ).abs() < 0.25);
    }

    /// Runs the example through its command-line entry point.
    #[test]
    fn example_entry_point_succeeds() {
        main();
    }
}
