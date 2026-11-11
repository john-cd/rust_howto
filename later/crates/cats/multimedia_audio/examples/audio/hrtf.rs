// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Renders a mono tone as binaural stereo with `hrtf`.
//!
//! A real application loads an HRIR sphere file from disk or embeds a small
//! fixture file. This example loads a small binary HRIR sphere fixture shipped
//! with the project via `include_bytes!`. The sphere contains four directions
//! (a tetrahedron), each with a one-sample impulse response per ear. The right
//! ear is attenuated, as if the source were on the left.

use std::error::Error;

use hrtf::HrirSphere;
use hrtf::HrtfContext;
use hrtf::HrtfProcessor;
use hrtf::Vec3;

const SAMPLE_RATE: u32 = 44_100;
const SOURCE_LEN: usize = 512;

/// HRIR sphere fixture bytes compiled into the binary.
const HRIR_FIXTURE_BYTES: &[u8] = include_bytes!("fixture.hrir");

/// Returns the root mean square (RMS) of a signal stream.
fn rms(samples: impl Iterator<Item = f32>) -> f32 {
    let (sum, count) = samples.fold((0.0, 0), |(sum, count), s| (sum + s * s, count + 1));
    (sum / count as f32).sqrt()
}

/// Spatializes a 440 Hz tone and compares the sound level in both ears.
fn main() -> Result<(), Box<dyn Error>> {
    let sphere = HrirSphere::new(HRIR_FIXTURE_BYTES, SAMPLE_RATE)
        .map_err(|e| format!("invalid HRIR sphere: {e:?}"))?;
    let mut processor = HrtfProcessor::new(sphere, SOURCE_LEN);

    // The source length must match the HRTF processor's configured frame length.
    let source: Vec<f32> = (0..SOURCE_LEN)
        .map(|i| {
            let time = i as f32 / SAMPLE_RATE as f32;
            (std::f32::consts::TAU * 440.0 * time).sin()
        })
        .collect();
    // The output is stereo: processed samples are mixed into it.
    let mut output = vec![(0.0_f32, 0.0_f32); source.len()];

    // The vector points from the listener to the source.
    let direction = Vec3::new(1.0, 1.0, 1.0);
    // The processor keeps the convolution history between frames of this source.
    let mut history = Vec::new();
    processor.process_samples(HrtfContext {
        source: &source,
        output: &mut output,
        new_sample_vector: direction,
        prev_sample_vector: direction,
        history: &mut history,
        // Equal gains: no distance fade within the frame.
        new_gain: 1.0,
        prev_gain: 1.0,
    });

    let left = rms(output.iter().map(|&(left, _)| left));
    let right = rms(output.iter().map(|&(_, right)| right));
    println!("left RMS {left:.3}, right RMS {right:.3}");
    // The sine has an RMS of about 0.7. The left ear hears most of it...
    assert!(left > 0.3);
    // ...and the right ear hears the source four times quieter.
    assert!((right / left - 0.25).abs() < 0.02);
    Ok(())
}
// ANCHOR_END: example

/// Runs the device-free HRTF example.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_binaural_stereo() -> Result<(), Box<dyn Error>> {
        main()
    }
}
