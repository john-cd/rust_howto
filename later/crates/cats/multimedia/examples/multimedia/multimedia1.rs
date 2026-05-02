#![allow(dead_code)]
// ANCHOR: example
//! Play a simple sine tone using the `rodio` audio crate.
//!
//! This example opens the default audio output device, generates a 440 Hz
//! sine wave, and plays it for two seconds.

use std::error::Error;
use std::time::Duration;

use rodio::OutputStream;
use rodio::Sink;
use rodio::source::SineWave;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let source = SineWave::new(440);
    sink.append(source.take_duration(Duration::from_secs(2)));
    sink.sleep_until_end();

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
        let _result = OutputStream::try_default();
    }
}
