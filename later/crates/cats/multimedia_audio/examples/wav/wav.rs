//! Writes and reads a WAV file through an in-memory cursor.

use std::error::Error;
use std::io::Cursor;

// ANCHOR: example
/// Writes one silent sample to an in-memory WAV and reads it back.
fn main() -> Result<(), Box<dyn Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 8_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut bytes = Vec::new();
    {
        let cursor = Cursor::new(&mut bytes);
        let mut writer = hound::WavWriter::new(cursor, spec)?;
        writer.write_sample(0_i16)?;
        writer.finalize()?;
    }

    let reader = hound::WavReader::new(Cursor::new(bytes))?;
    assert_eq!(reader.duration(), 1);
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies a finalized in-memory WAV contains the sample written to it.
    #[test]
    fn writes_and_reads_wav_without_an_audio_device()
    -> Result<(), Box<dyn Error>> {
        main()
    }
}
