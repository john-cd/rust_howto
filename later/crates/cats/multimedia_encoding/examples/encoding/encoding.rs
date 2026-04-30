#![allow(dead_code)]
// ANCHOR: example
//! Encode a blank AV1 frame using the `rav1e` crate.
//!
//! This example creates a default encoder context, sends one frame, flushes the
//! encoder, and counts the packets emitted by the encoder.

use std::error::Error;

use rav1e::prelude::*;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cfg = Config::default();
    let mut ctx: Context<u8> = cfg.new_context()?;

    let frame = ctx.new_frame();
    ctx.send_frame(frame)?;
    ctx.flush();

    let mut packet_count = 0;
    loop {
        match ctx.receive_packet() {
            Ok(_packet) => packet_count += 1,
            Err(EncoderStatus::Encoded) => (),
            Err(EncoderStatus::LimitReached) => break,
            Err(status) => {
                return Err(format!("Encoding failed: {:?}", status).into());
            }
        }
    }

    println!("Encoded {} AV1 packet(s) from one frame.", packet_count);
    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_creates_context() {
        let cfg = Config::default();
        let ctx: Context<u8> =
            cfg.new_context().expect("default context should build");
        let _frame = ctx.new_frame();
    }
}
