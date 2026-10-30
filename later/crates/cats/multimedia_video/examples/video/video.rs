// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encodes synthetic YUV video frames into AV1 packets using `rav1e`.

use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use rav1e::prelude::Config;
use rav1e::prelude::EncoderConfig;
use rav1e::prelude::EncoderStatus;

/// Encodes a sequence of synthetic YUV frames of the given dimensions.
///
/// Returns the total number of encoded AV1 packets produced by the encoder
/// context.
fn encode_synthetic_frames(
    width: usize,
    height: usize,
    frame_count: usize,
) -> Result<usize> {
    // Configure speed preset 10 for rapid, low-complexity encoding in real-time
    // or test scenarios.
    let mut encoder_config = EncoderConfig::with_speed_preset(10);
    encoder_config.width = width;
    encoder_config.height = height;

    let config = Config::default().with_encoder_config(encoder_config);

    // Initializing the context allocates internal memory pools, rate control
    // metadata, and thread state for 8-bit YUV color depth based on the encoder
    // configuration.
    let mut context = config.new_context::<u8>()?;

    for frame_idx in 0..frame_count {
        // Allocating a frame buffer from the context ensures plane strides and
        // memory alignment match encoder needs.
        let mut frame = context.new_frame();

        // Fill Y, U, and V color planes with animated synthetic pixel values.
        // The Y (luminance) plane varies over time, while U and V (chrominance)
        // remain neutral gray.
        let luma_value = u8::try_from((128 + frame_idx * 20) % 256)?;

        for (plane_idx, plane) in frame.planes.iter_mut().enumerate() {
            let plane_width = plane.cfg.width;
            let plane_height = plane.cfg.height;
            let fill_val = if plane_idx == 0 { luma_value } else { 128 };

            for row in plane.rows_iter_mut() {
                row.fill(fill_val);
            }

            // Video encoders process pixel blocks in fixed tiles (e.g., 64x64
            // superblock boundaries). Padding duplicates border pixels into
            // boundary areas to prevent edge artifact read overflows.
            plane.pad(plane_width, plane_height);
        }

        context.send_frame(frame)?;
    }

    // Flush signals the encoder that no further frames will be sent, allowing
    // remaining queued packets to be produced.
    context.flush();

    let mut packet_count = 0;
    loop {
        match context.receive_packet() {
            Ok(packet) => {
                packet_count += 1;
                println!(
                    "Encoded AV1 packet #{packet_count} ({} bytes, frame type: {}).",
                    packet.data.len(),
                    packet.frame_type
                );
            }
            Err(EncoderStatus::Encoded) => {}
            Err(EncoderStatus::LimitReached) => break,
            Err(status) => {
                bail!("AV1 encoder stopped unexpectedly: {status:?}")
            }
        }
    }

    ensure!(
        packet_count > 0,
        "encoding {frame_count} frames did not produce any AV1 packets"
    );
    Ok(packet_count)
}

/// Runs the deterministic AV1 video encoder example.
fn main() -> Result<()> {
    let width = 64;
    let height = 64;
    let frame_count = 3;

    let packet_count = encode_synthetic_frames(width, height, frame_count)?;
    println!(
        "Successfully encoded {frame_count} frame(s) into {packet_count} AV1 packet(s)."
    );
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_synthetic_frames() {
        let packet_count = encode_synthetic_frames(64, 64, 3)
            .expect("synthetic frames should encode successfully");
        assert!(packet_count > 0);
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
