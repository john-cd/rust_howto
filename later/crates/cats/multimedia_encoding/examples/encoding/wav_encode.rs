// ANCHOR: example
//! Encode PCM audio samples to a WAV buffer using `hound`.

use std::error::Error;
use std::io::Cursor;

use hound::SampleFormat;
use hound::WavReader;
use hound::WavSpec;
use hound::WavWriter;

/// Encodes generated PCM sine wave samples into a WAV stream in memory.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut cursor, spec)?;

    // Write 100 samples of a 440 Hz sine wave.
    for i in 0..100 {
        let t = f64::from(i) / 44100.0;
        let sample = (t * 440.0 * 2.0 * std::f64::consts::TAU).sin();
        let amplitude = (sample * f64::from(i16::MAX)) as i16;
        writer.write_sample(amplitude)?;
    }
    writer.finalize()?;

    let wav_data = cursor.into_inner();
    let reader = WavReader::new(Cursor::new(&wav_data))?;
    println!(
        "Encoded {} WAV samples ({} total bytes).",
        reader.len(),
        wav_data.len()
    );

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wav_encode_roundtrip() {
        main().expect("WAV encoding example should succeed");
    }
}
