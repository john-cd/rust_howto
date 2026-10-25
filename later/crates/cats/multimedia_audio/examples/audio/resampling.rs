#![allow(dead_code)]
// ANCHOR: example
//! Resamples a mono buffer from 48 kHz to 44.1 kHz with `rubato`.

use std::error::Error;

use rubato::Fft;
use rubato::FixedSync;
use rubato::Resampler;
use rubato::audioadapter_buffers::direct::SequentialSliceOfVecs;

/// Converts one generated mono chunk to a different sample rate.
fn resample_chunk() -> Result<(usize, Vec<f32>), Box<dyn Error>> {
    let mut resampler = Fft::<f32>::new(48_000, 44_100, 1_024, 1, FixedSync::Both)?;
    let input_frames = resampler.input_frames_next();
    let input = (0..input_frames)
        .map(|frame| {
            let time = frame as f32 / 48_000.0;
            (std::f32::consts::TAU * 440.0 * time).sin()
        })
        .collect::<Vec<_>>();
    let channels = [input];
    let input_adapter = SequentialSliceOfVecs::new(&channels, 1, input_frames)?;
    let output = resampler.process(&input_adapter, None)?;
    // The output is a single interleaved channel.
    Ok((input_frames, output.take_data()))
}

/// Resamples a chunk and checks the output buffer.
fn main() -> Result<(), Box<dyn Error>> {
    let (input_frames, samples) = resample_chunk()?;
    assert!(!samples.is_empty());
    assert!(samples.len() < input_frames);
    println!("Resampled {input_frames} frames to {}.", samples.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs the device-free resampling example.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the output frame count shrinks when converting 48 kHz to 44.1
    /// kHz.
    #[test]
    fn resamples_without_an_audio_device() -> Result<(), Box<dyn Error>> {
        let (input_frames, samples) = resample_chunk()?;
        assert!(!samples.is_empty());
        assert!(samples.len() < input_frames);
        assert!(samples.iter().all(|sample| sample.is_finite()));
        Ok(())
    }
}
