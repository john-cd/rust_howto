// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Writes one second of a 440 Hz sine wave to an in-memory WAV file with
//! `hound`, then reads it back.

use std::error::Error;
use std::f32::consts::TAU;
use std::io::Cursor;

use hound::SampleFormat;
use hound::WavReader;
use hound::WavSpec;
use hound::WavWriter;

/// Samples per second.
const SAMPLE_RATE: u32 = 44_100;

/// Encodes one second of a 440 Hz sine wave as a 16-bit mono WAV file.
fn encode_sine(spec: WavSpec) -> Result<Vec<u8>, Box<dyn Error>> {
    // `WavWriter` needs `Write + Seek` to patch the header at the end:
    // a `Cursor` over a `Vec<u8>` provides both. A `BufWriter<File>` works
    // the same way and streams the samples to disk without holding them all.
    let mut cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut cursor, spec)?;
    for n in 0..SAMPLE_RATE {
        let seconds = n as f32 / SAMPLE_RATE as f32;
        let value = (TAU * 440.0 * seconds).sin();
        // Scale to the 16-bit range, leaving some headroom.
        writer.write_sample((value * f32::from(i16::MAX) * 0.5) as i16)?;
    }
    // `finalize` writes the sizes in the header: do not forget it.
    writer.finalize()?;
    Ok(cursor.into_inner())
}

/// Writes the WAV file, reads it back and checks the format and content.
fn main() -> Result<(), Box<dyn Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let bytes = encode_sine(spec)?;

    let mut reader = WavReader::new(Cursor::new(bytes))?;
    assert_eq!(reader.spec(), spec);
    // `duration` is the number of samples per channel.
    assert_eq!(reader.duration(), SAMPLE_RATE);

    let samples = reader.samples::<i16>().collect::<Result<Vec<_>, _>>()?;
    assert_eq!(samples.len(), SAMPLE_RATE as usize);
    // The wave starts at zero and reaches half the full scale.
    assert_eq!(samples[0], 0);
    let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
    assert!(peak > 16_000 && peak <= 0x4000);
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the wav write read example through its entry point.
    #[test]
    fn test_wav_write_read_example_entry_point() -> Result<(), Box<dyn Error>> {
        main()
    }
}
