#![allow(dead_code)]
// ANCHOR: example
//! Renders a mono tone as binaural stereo with `hrtf`.
//!
//! A real application loads an HRIR sphere file from disk. This example builds
//! a tiny sphere in memory, in the file format that `HrirSphere::new` reads,
//! so it needs no download: four directions (a tetrahedron), each with a
//! one-sample impulse response per ear. The right ear is attenuated, as if
//! the source were on the left.

use std::error::Error;

use hrtf::HrirSphere;
use hrtf::HrtfContext;
use hrtf::HrtfProcessor;
use hrtf::Vec3;

const SAMPLE_RATE: u32 = 44_100;
/// Length of every head-related impulse response, in samples.
const HRIR_LEN: usize = 512;
/// Position of the impulse within each response. The loader resamples the
/// responses with a sinc filter that delays them, so an impulse at the very
/// start (or in a short response) would be smeared or cut off.
const IMPULSE_INDEX: usize = 64;
const BLOCK_LEN: usize = 128;
const INTERPOLATION_STEPS: usize = 4;

/// Serializes a sphere: magic, sample rate, HRIR length, vertex count, index
/// count, face indices, then for each vertex its position and both HRIRs
/// (little endian).
fn tetrahedron_sphere_bytes(left_gain: f32, right_gain: f32) -> Vec<u8> {
    let vertices = [
        [1.0_f32, 1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [1.0, -1.0, -1.0],
    ];
    let faces = [[0_u32, 1, 2], [0, 3, 1], [0, 2, 3], [1, 3, 2]];

    let mut bytes = b"HRIR".to_vec();
    for value in [
        SAMPLE_RATE,
        HRIR_LEN as u32,
        vertices.len() as u32,
        (faces.len() * 3) as u32,
    ] {
        bytes.extend(value.to_le_bytes());
    }
    for index in faces.iter().flatten() {
        bytes.extend(index.to_le_bytes());
    }
    for position in vertices {
        for coordinate in position {
            bytes.extend(coordinate.to_le_bytes());
        }
        for gain in [left_gain, right_gain] {
            // A single impulse scales the signal.
            for i in 0..HRIR_LEN {
                let sample = if i == IMPULSE_INDEX { gain } else { 0.0_f32 };
                bytes.extend(sample.to_le_bytes());
            }
        }
    }
    bytes
}

/// Returns the root mean square of a signal.
fn rms(samples: impl Iterator<Item = f32>) -> f32 {
    let (sum, count) =
        samples.fold((0.0, 0), |(sum, count), s| (sum + s * s, count + 1));
    (sum / count as f32).sqrt()
}

/// Spatializes a 440 Hz tone and compares the two ears.
fn main() -> Result<(), Box<dyn Error>> {
    let bytes = tetrahedron_sphere_bytes(1.0, 0.25);
    let sphere = HrirSphere::new(bytes.as_slice(), SAMPLE_RATE)
        .map_err(|e| format!("invalid HRIR sphere: {e:?}"))?;
    let mut processor =
        HrtfProcessor::new(sphere, INTERPOLATION_STEPS, BLOCK_LEN);

    // The source must be exactly `interpolation_steps * block_len` samples.
    let source: Vec<f32> = (0..INTERPOLATION_STEPS * BLOCK_LEN)
        .map(|i| {
            let time = i as f32 / SAMPLE_RATE as f32;
            (std::f32::consts::TAU * 440.0 * time).sin()
        })
        .collect();
    // The output is stereo: processed samples are mixed into it.
    let mut output = vec![(0.0_f32, 0.0_f32); source.len()];

    // The vector points from the listener to the source.
    let direction = Vec3::new(1.0, 1.0, 1.0);
    // The processor keeps the tail of the convolution between calls; reuse
    // these buffers for the next block of the same source.
    let mut prev_left = Vec::new();
    let mut prev_right = Vec::new();
    processor.process_samples(HrtfContext {
        source: &source,
        output: &mut output,
        new_sample_vector: direction,
        prev_sample_vector: direction,
        prev_left_samples: &mut prev_left,
        prev_right_samples: &mut prev_right,
        // Equal gains: no distance fade within the block.
        new_distance_gain: 1.0,
        prev_distance_gain: 1.0,
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
