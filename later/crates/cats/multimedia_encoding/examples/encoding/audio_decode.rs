// ANCHOR: example
//! Probe and decode audio formats to linear PCM samples using `symphonia`.

use std::error::Error;
use std::io::Cursor;

use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Probes an in-memory WAV byte buffer and decodes audio frames into PCM samples.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let sample_count_written = 128;
    let wav_bytes = create_sample_wav(sample_count_written)?;
    let mss = MediaSourceStream::new(Box::new(Cursor::new(wav_bytes)), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");

    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or("no default audio track found")?;
    let mut decoder =
        symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    let mut total_frames = 0;
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(err))
                if err.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(err) => return Err(err.into()),
        };

        let decoded = decoder.decode(&packet)?;
        total_frames += decoded.frames();
    }

    assert_eq!(
        total_frames, sample_count_written,
        "decoded frame count must match written sample count"
    );

    println!("Decoded {total_frames} audio samples with symphonia.");
    Ok(())
}

/// Helper function to build a valid WAV buffer in memory for decoding.
fn create_sample_wav(sample_count: usize) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut cursor = Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
    for _ in 0..sample_count {
        writer.write_sample(0i16)?;
    }
    writer.finalize()?;
    Ok(cursor.into_inner())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_decode_entry_point() {
        main().expect("Audio decoding entry point should succeed");
    }
}
