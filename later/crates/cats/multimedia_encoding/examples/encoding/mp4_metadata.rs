// ANCHOR: example
//! Inspect MP4 container metadata including tracks and duration using `mp4`.
//!
//! MP4 is a container: it holds separate tracks (video, audio, subtitles) plus
//! metadata such as duration and codecs. Reading the metadata is cheap because
//! the index is stored apart from the media data, so you need not decode the
//! video.

use std::error::Error;
use std::io::Cursor;

/// Reads track and duration information from an MP4 stream using `mp4`.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mp4_data = create_sample_mp4()?;
    let size = mp4_data.len() as u64;
    let reader = Cursor::new(mp4_data);

    let mp4 = mp4::Mp4Reader::read_header(reader, size)?;
    let duration = mp4.duration();
    let track_count = mp4.tracks().len();

    println!(
        "MP4 container contains {track_count} track(s) with duration {duration:?}."
    );

    #[allow(
        clippy::iter_over_hash_type,
        reason = "the track order does not matter for this report"
    )]
    for (track_id, track) in mp4.tracks() {
        println!(
            "Track #{track_id}: type={:?}, media_duration={:?}",
            track.track_type()?,
            track.duration()
        );
    }

    Ok(())
}

/// Generates a minimal valid MP4 container in memory.
fn create_sample_mp4() -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let mut buf = Vec::new();
    let config = mp4::Mp4Config {
        major_brand: str::parse("isom")?,
        minor_version: 512,
        compatible_brands: vec![str::parse("isom")?, str::parse("iso2")?],
        timescale: 1000,
    };
    let mut writer =
        mp4::Mp4Writer::write_start(Cursor::new(&mut buf), &config)?;

    let media_conf = mp4::TrackConfig {
        track_type: mp4::TrackType::Audio,
        timescale: 1000,
        language: "und".to_owned(),
        media_conf: mp4::MediaConfig::AacConfig(mp4::AacConfig {
            bitrate: 128_000,
            profile: mp4::AudioObjectType::AacLowComplexity,
            freq_index: mp4::SampleFreqIndex::Freq44100,
            chan_conf: mp4::ChannelConfig::Mono,
        }),
    };
    writer.add_track(&media_conf)?;

    let sample = mp4::Mp4Sample {
        start_time: 0,
        duration: 1000,
        rendering_offset: 0,
        is_sync: true,
        bytes: mp4::Bytes::from(vec![0u8; 10]),
    };
    writer.write_sample(1, &sample)?;
    writer.write_end()?;

    drop(writer);
    Ok(buf)
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mp4_metadata_entry_point() {
        main().expect("MP4 metadata entry point should succeed");
    }
}
