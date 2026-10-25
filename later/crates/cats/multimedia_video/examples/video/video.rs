#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Encodes a synthetic gray frame as an AV1 packet using `rav1e`.

use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use rav1e::prelude::Config;
use rav1e::prelude::EncoderConfig;
use rav1e::prelude::EncoderStatus;

/// Encodes one synthetic 64-by-64 frame and returns the number of packets.
fn encode_synthetic_frame() -> Result<usize> {
    let width = 64;
    let height = 64;
    let mut encoder_config = EncoderConfig::with_speed_preset(10);
    encoder_config.width = width;
    encoder_config.height = height;

    let config = Config::default().with_encoder_config(encoder_config);
    let mut context = config.new_context::<u8>()?;
    let mut frame = context.new_frame();

    // Fill the visible samples of each YUV plane with mid-range gray, then
    // extend the edges into the padding required by the encoder.
    for plane in &mut frame.planes {
        let plane_width = plane.cfg.width;
        let plane_height = plane.cfg.height;
        for row in plane.rows_iter_mut() {
            row.fill(128);
        }
        plane.pad(plane_width, plane_height);
    }

    context.send_frame(frame)?;
    context.flush();

    let mut packet_count = 0;
    loop {
        match context.receive_packet() {
            Ok(packet) => {
                packet_count += 1;
                println!(
                    "Encoded an AV1 packet ({} bytes).",
                    packet.data.len()
                );
            }
            Err(EncoderStatus::Encoded) => {}
            Err(EncoderStatus::LimitReached) => break,
            Err(status) => {
                bail!("AV1 encoder stopped unexpectedly: {status:?}")
            }
        }
    }

    ensure!(packet_count > 0, "the frame did not produce an AV1 packet");
    Ok(packet_count)
}

/// Runs the deterministic encoder example.
fn main() -> Result<()> {
    let packet_count = encode_synthetic_frame()?;
    println!("Encoded {packet_count} AV1 packet(s).");
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

    /// Verifies that a synthetic frame produces encoded AV1 output offline.
    #[test]
    fn encodes_synthetic_frame() {
        let packet_count =
            encode_synthetic_frame().expect("synthetic frame should encode");
        assert!(packet_count > 0);
    }
}
// [review; TODO call main??](https://github.com/john-cd/rust_howto/issues/809)
