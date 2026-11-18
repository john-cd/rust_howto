// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Measures a mono frame with a periodic Hann window and a one-sided FFT.

use std::error::Error;
use std::sync::Arc;

use rustfft::Fft;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

/// A reusable FFT plan with fixed frame length and sampling rate.
struct Analyzer {
    /// Reusable forward transform sized for the input frame.
    fft: Arc<dyn Fft<f32>>,
    /// Periodic Hann weights applied to the frame before transforming it.
    window: Vec<f32>,
    /// Input sampling rate in hertz.
    sample_rate: f32,
}

/// Linear peak amplitudes with frequency spacing in hertz.
struct Spectrum {
    /// One-sided linear peak amplitudes, indexed from the DC bin.
    amplitudes: Vec<f32>,
    /// Frequency in hertz between adjacent bins.
    bin_hz: f32,
}

impl Analyzer {
    /// Plans an FFT for at least four samples and a positive finite sample rate.
    fn new(length: usize, sample_rate: f32) -> Result<Self, &'static str> {
        if !(4..=65_536).contains(&length) || !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Err("invalid frame length or sample rate");
        }
        let fft = FftPlanner::new().plan_fft_forward(length);
        let window = (0..length)
            .map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / length as f32).cos())
            .collect();
        Ok(Self {
            fft,
            window,
            sample_rate,
        })
    }

    /// Analyzes a finite mono frame off the realtime callback thread.
    fn analyze(&self, samples: &[f32]) -> Result<Spectrum, &'static str> {
        if samples.len() != self.window.len() || samples.iter().any(|v| !v.is_finite()) {
            return Err("frame length mismatch or non-finite sample");
        }
        let mut buffer: Vec<_> = samples
            .iter()
            .zip(&self.window)
            .map(|(sample, weight)| Complex::new(sample * weight, 0.0))
            .collect();
        self.fft.process(&mut buffer);
        // The window sum restores the peak amplitude of a bin-centered tone.
        let gain: f32 = self.window.iter().sum();
        let amplitudes: Vec<_> = buffer[..=samples.len() / 2]
            .iter()
            .enumerate()
            .map(|(bin, value)| {
                // Interior bins have a negative-frequency partner. DC and Nyquist stand alone.
                let edge =
                    bin == 0 || (samples.len().is_multiple_of(2) && bin == samples.len() / 2);
                value.norm() / gain * if edge { 1.0 } else { 2.0 }
            })
            .collect();
        if amplitudes.iter().any(|v| !v.is_finite()) {
            return Err("FFT magnitude overflow");
        }
        Ok(Spectrum {
            amplitudes,
            bin_hz: self.sample_rate / samples.len() as f32,
        })
    }
}

impl Spectrum {
    /// Returns the dominant bin in hertz; silence has no defined peak.
    fn peak_hz(&self) -> Option<f32> {
        self.amplitudes
            .iter()
            .enumerate()
            .filter(|(_, value)| **value > 1e-6)
            .max_by(|a, b| a.1.total_cmp(b.1))
            .map(|(bin, _)| bin as f32 * self.bin_hz)
    }
}

/// Prints the peak of a generated tone without using an audio device or file.
fn main() -> Result<(), Box<dyn Error>> {
    let analyzer = Analyzer::new(1024, 8192.0)?;
    let samples: Vec<_> = (0..1024)
        .map(|i| 0.5 * (std::f32::consts::TAU * 128.0 * i as f32 / 1024.0).sin())
        .collect();
    println!("Peak: {:?} Hz", analyzer.analyze(&samples)?.peak_hz());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// A bin-centered tone retains its frequency and coherent peak amplitude.
    #[test]
    fn tone_frequency_and_amplitude() {
        let analyzer = Analyzer::new(1024, 8192.0).expect("valid analyzer");
        let samples: Vec<_> = (0..1024)
            .map(|i| 0.5 * (std::f32::consts::TAU * 128.0 * i as f32 / 1024.0).sin())
            .collect();
        let spectrum = analyzer.analyze(&samples).expect("finite tone");
        assert_eq!(spectrum.peak_hz(), Some(1024.0));
        assert!((spectrum.amplitudes[128] - 0.5).abs() < 1e-4);
    }

    /// Silence has no peak and produces no NaN normalization artifacts.
    #[test]
    fn silence_has_no_pitch() {
        let analyzer = Analyzer::new(32, 48_000.0).expect("valid analyzer");
        let spectrum = analyzer.analyze(&[0.0; 32]).expect("silent frame");
        assert_eq!(spectrum.peak_hz(), None);
        assert!(spectrum.amplitudes.iter().all(|v| *v == 0.0));
    }

    /// Nyquist amplitude is not doubled like interior one-sided bins.
    #[test]
    fn nyquist_scaling() {
        let analyzer = Analyzer::new(32, 48_000.0).expect("valid analyzer");
        let samples: Vec<_> = (0..32)
            .map(|i| if i % 2 == 0 { 0.25 } else { -0.25 })
            .collect();
        let spectrum = analyzer.analyze(&samples).expect("finite Nyquist signal");
        assert!((spectrum.amplitudes[16] - 0.25).abs() < 1e-6);
    }

    /// Invalid rates, frame sizes, and non-finite samples are rejected.
    #[test]
    fn invalid_input_is_reported() {
        assert!(Analyzer::new(0, 48_000.0).is_err());
        assert!(Analyzer::new(8, f32::NAN).is_err());
        let analyzer = Analyzer::new(8, 48_000.0).expect("valid analyzer");
        assert!(analyzer.analyze(&[0.0; 7]).is_err());
        assert!(analyzer.analyze(&[f32::INFINITY; 8]).is_err());
    }
}
