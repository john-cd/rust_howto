// ANCHOR: example
//! Probe and decode audio formats to linear PCM samples using `symphonia`.
//!
//! Audio files come in many formats (MP3, FLAC, Ogg, WAV). A decoder first
//! probes the container to detect the format, then converts the compressed data
//! to PCM (pulse-code modulation), the plain list of samples that sound
//! hardware and audio processing use.

use std::error::Error;
use std::io::Cursor;

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::TrackType;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

/// Probes an in-memory WAV byte buffer and decodes audio frames into PCM
/// samples.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let sample_count_written = 128;
    let wav_bytes = create_sample_wav(sample_count_written)?;
    let mss = MediaSourceStream::new(
        Box::new(Cursor::new(wav_bytes)),
        Default::default(),
    );

    let mut hint = Hint::new();
    hint.with_extension("wav");

    let mut format = symphonia::default::get_probe().probe(
        &hint,
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    )?;

    let track = format
        .default_track(TrackType::Audio)
        .ok_or("no default audio track found")?;
    let codec_params = track
        .codec_params
        .as_ref()
        .and_then(|params| params.audio())
        .ok_or("the default track has no audio codec parameters")?;
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &AudioDecoderOptions::default())?;

    let mut total_frames = 0;
    // `next_packet` returns `Ok(None)` at the end of the stream.
    while let Some(packet) = format.next_packet()? {
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
fn create_sample_wav(
    sample_count: usize,
) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
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
