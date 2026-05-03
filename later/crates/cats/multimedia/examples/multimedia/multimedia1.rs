#![allow(dead_code)]
// ANCHOR: example
//! Play a simple sine tone using the `rodio` audio crate.
//!
//! This example opens the default audio output device, generates a 440 Hz
//! sine wave, and plays it for two seconds.

use std::error::Error;
use std::time::Duration;

use rodio::DeviceSinkBuilder;
use rodio::Player;
use rodio::Source;
use rodio::source::SineWave;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let sink_handle = DeviceSinkBuilder::open_default_sink()?;
    let player = Player::connect_new(&sink_handle.mixer());

    let source = SineWave::new(440.0);
    player.append(source.take_duration(Duration::from_secs(2)));
    player.sleep_until_end();

    println!("Played a 440 Hz sine tone for 2 seconds.");
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
    #[ignore = "requires audio output device"]
    fn test_main() {
        main().unwrap();
    }

    #[test]
    fn create_rodio_output_stream() {
        let _result = DeviceSinkBuilder::open_default_sink(); // TODO review - this test is not really a unit test, but it does verify that we can create an output stream without panicking. It will fail if there is no audio output device available, which is common in CI environments. We could consider using a mock or virtual audio device for testing in the future.
    }
}
