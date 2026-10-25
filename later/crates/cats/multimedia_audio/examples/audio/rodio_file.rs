#![allow(dead_code)]
// ANCHOR: example
//! Decodes a WAV in memory and shows how to play a file with `rodio`.

use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use hound::SampleFormat;
use hound::WavSpec;
use hound::WavWriter;
use rodio::Decoder;
use rodio::DeviceSinkBuilder;
use rodio::Player;

/// Creates a short mono WAV in memory for the device-free decoder example.
fn make_wav() -> Result<Vec<u8>, Box<dyn Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 8_000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut cursor, spec)?;
    for sample in 0..800 {
        let phase = std::f32::consts::TAU * 440.0 * sample as f32 / 8_000.0;
        writer
            .write_sample((phase.sin() * f32::from(i16::MAX) * 0.5) as i16)?;
    }
    writer.finalize()?;
    Ok(cursor.into_inner())
}

/// Decodes WAV bytes without opening an audio device.
fn decode_wav(bytes: Vec<u8>) -> Result<Vec<f32>, Box<dyn Error>> {
    let decoder = Decoder::try_from(Cursor::new(bytes))?;
    Ok(decoder.collect())
}

/// Plays an audio file through the system's default output device.
fn play_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let sink = DeviceSinkBuilder::open_default_sink()?;
    let player = Player::connect_new(sink.mixer());
    player.append(Decoder::try_from(File::open(path)?)?);
    player.sleep_until_end();
    Ok(())
}

/// Decodes a generated WAV and prints how to play an audio file.
fn main() -> Result<(), Box<dyn Error>> {
    let samples = decode_wav(make_wav()?)?;
    assert_eq!(samples.len(), 800);
    println!(
        "Decoded {} samples; call play_file(path) to hear an audio file.",
        samples.len()
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the device-free decoder example.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies WAV decoding returns samples without requiring an audio device.
    #[test]
    fn decodes_wav_without_an_audio_device() -> Result<(), Box<dyn Error>> {
        assert_eq!(decode_wav(make_wav()?)?.len(), 800);
        Ok(())
    }
}
