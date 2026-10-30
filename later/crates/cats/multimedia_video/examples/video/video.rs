// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encodes a short synthetic video as AV1 packets using `rav1e`.
//!
//! `rav1e` is a pure-Rust AV1 encoder. The workflow is the same for any
//! frame source: configure an encoder, send frames in display order, flush,
//! then drain the packets. Each packet is a chunk of the AV1 bitstream that a
//! muxer (for example, into an MP4 or WebM container) would store.

use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use rav1e::prelude::Config;
use rav1e::prelude::Context;
use rav1e::prelude::EncoderConfig;
use rav1e::prelude::EncoderStatus;

/// Describes the synthetic video to encode.
struct VideoSpec {
    /// Width in pixels.
    width: usize,
    /// Height in pixels.
    height: usize,
    /// Number of frames to encode.
    frames: u64,
}

/// Fills the next frame with a horizontal luma gradient that shifts by
/// `shift` samples, so that consecutive frames differ.
fn send_frame(context: &mut Context<u8>, shift: usize) -> Result<()> {
    // The frame is allocated by the context, which knows the pixel format,
    // the chroma subsampling and the padding that this encoder needs.
    let mut frame = context.new_frame();
    for (index, plane) in frame.planes.iter_mut().enumerate() {
        let plane_width = plane.cfg.width;
        let plane_height = plane.cfg.height;
        for row in plane.rows_iter_mut() {
            for (x, sample) in row.iter_mut().enumerate() {
                // Plane 0 is luma; planes 1 and 2 are chroma, left neutral.
                *sample = if index == 0 {
                    ((x + shift) % 256) as u8
                } else {
                    128
                };
            }
        }
        // The encoder reads blocks that extend past the visible area, so the
        // edge samples are replicated into the padding.
        plane.pad(plane_width, plane_height);
    }
    context.send_frame(frame)?;
    Ok(())
}

/// Encodes the synthetic video and returns the number of packets produced.
fn encode(spec: &VideoSpec) -> Result<usize> {
    // Speed preset 10 is the fastest and the least efficient.
    let mut encoder_config = EncoderConfig::with_speed_preset(10);
    encoder_config.width = spec.width;
    encoder_config.height = spec.height;

    let config = Config::default().with_encoder_config(encoder_config);
    // The context owns the encoder state; the sample type (`u8`) matches the
    // 8-bit pixels written above.
    let mut context = config.new_context::<u8>()?;

    for frame_index in 0..spec.frames {
        send_frame(&mut context, frame_index as usize * 4)?;
    }
    // Tell the encoder that no more frames follow so it emits what it holds.
    context.flush();

    let mut packet_count = 0;
    loop {
        match context.receive_packet() {
            Ok(packet) => {
                packet_count += 1;
                println!(
                    "Packet {} for frame {}: {} bytes.",
                    packet_count,
                    packet.input_frameno,
                    packet.data.len()
                );
            }
            // The encoder consumed input without producing output yet.
            Err(EncoderStatus::Encoded) => {}
            // All packets have been drained after the flush.
            Err(EncoderStatus::LimitReached) => break,
            Err(status) => {
                bail!("AV1 encoder stopped unexpectedly: {status:?}")
            }
        }
    }

    ensure!(packet_count > 0, "the frames did not produce an AV1 packet");
    Ok(packet_count)
}

/// Encodes ten 64-by-64 frames.
fn main() -> Result<()> {
    let spec = VideoSpec {
        width: 64,
        height: 64,
        frames: 10,
    };
    let packet_count = encode(&spec)?;
    println!(
        "Encoded {} frame(s) into {packet_count} AV1 packet(s).",
        spec.frames
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every frame of a short video is encoded into a packet, offline.
    #[test]
    fn encodes_every_frame() {
        let spec = VideoSpec {
            width: 64,
            height: 64,
            frames: 5,
        };
        let packet_count =
            encode(&spec).expect("synthetic frames should encode");
        assert_eq!(packet_count, 5);
    }

    /// A frame size the encoder cannot use is reported as an error.
    #[test]
    fn rejects_an_empty_frame_size() {
        let spec = VideoSpec {
            width: 0,
            height: 0,
            frames: 1,
        };
        assert!(encode(&spec).is_err());
    }

    /// The example runs through its entry point.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
