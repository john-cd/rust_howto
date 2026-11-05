//! TODO Reviewed ??-??-2026.

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

/// Fills the next frame with a horizontal luma gradient that shifts by
/// `shift` samples, so that consecutive frames differ.
fn send_frame(context: &mut Context<u8>, shift: usize) -> Result<()> {
    // Creating the frame via `context.new_frame()` ensures that the frame
    // buffer matches the encoder context configuration, including the pixel
    // format, color space, chroma subsampling, and internal memory padding.
    // Video codecs like AV1 process blocks (such as 64x64 or 128x128 superblocks)
    // that extend beyond the visible video boundary, requiring padded borders.
    let mut frame = context.new_frame();
    for (index, plane) in frame.planes.iter_mut().enumerate() {
        let plane_width = plane.cfg.width;
        let plane_height = plane.cfg.height;
        for row in plane.rows_iter_mut() {
            for (x, sample) in row.iter_mut().enumerate() {
                // Plane 0 is luma (brightness); planes 1 and 2 are chroma (color difference).
                // Luma is varied over time to create visible motion, while chroma is neutral (128).
                *sample = if index == 0 {
                    ((x + shift) % 256) as u8
                } else {
                    128
                };
            }
        }
        // The encoder operates on blocks that often extend beyond the visible canvas.
        // Replicating border pixels into the surrounding padding prevents compression
        // artifacts at image edges.
        plane.pad(plane_width, plane_height);
    }
    context.send_frame(frame)?;
    Ok(())
}

/// Encodes a synthetic video of the given dimensions and frame count,
/// returning the number of packets produced.
fn encode(width: usize, height: usize, frames: u64) -> Result<usize> {
    // Speed preset 10 represents the fastest encoding tier with the lowest
    // compression efficiency, making it ideal for quick test runs and real-time streaming.
    // AV1 encoders evaluate multiple prediction modes, transform sizes, and motion vectors;
    // higher speed presets disable the most computationally expensive search passes.
    let mut encoder_config = EncoderConfig::with_speed_preset(10);
    encoder_config.width = width;
    encoder_config.height = height;

    // Wrap the low-level encoder settings in an immutable overall configuration.
    let config = Config::default().with_encoder_config(encoder_config);

    // Initialize the encoder context from the configuration. The context maintains the
    // internal encoder state machine, reference frame buffers, rate control models, and
    // packet queuing. The generic type parameter `u8` specifies 8-bit color depth per channel.
    let mut context = config.new_context::<u8>()?;

    for frame_index in 0..frames {
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
    let width = 64;
    let height = 64;
    let frames = 10;
    let packet_count = encode(width, height, frames)?;
    println!("Encoded {frames} frame(s) into {packet_count} AV1 packet(s).");
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
        let packet_count = encode(64, 64, 5).expect("synthetic frames should encode");
        assert_eq!(packet_count, 5);
    }

    /// Verifies encoding with non-square custom frame dimensions passed as arguments.
    #[test]
    fn encodes_custom_dimensions() {
        let packet_count = encode(80, 48, 2).expect("custom dimensions should encode");
        assert_eq!(packet_count, 2);
    }

    /// A frame size the encoder cannot use is reported as an error.
    #[test]
    fn rejects_an_empty_frame_size() {
        assert!(encode(0, 0, 1).is_err());
    }

    /// The example runs through its entry point.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
