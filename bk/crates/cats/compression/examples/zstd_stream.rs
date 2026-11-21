// TODO Reviewed ??-??-2026.
//! Streams bytes through a Zstandard encoder and decoder.

// ANCHOR: example
use std::io::Read;
use std::io::Write;
use std::io::{self};

/// Copies input into a Zstandard frame and returns the finished output writer.
///
/// # Errors
///
/// Returns an I/O error if reading, writing, or finishing the frame fails.
fn compress<W: Write>(mut input: impl Read, output: W) -> io::Result<W> {
    let mut encoder = zstd::stream::write::Encoder::new(output, 3)?;
    io::copy(&mut input, &mut encoder)?;
    encoder.finish()
}

/// Decodes the available Zstandard frames into the output writer.
///
/// # Errors
///
/// Returns an error for malformed compressed input or failed output.
fn decompress(input: impl Read, mut output: impl Write) -> io::Result<()> {
    let mut decoder = zstd::stream::read::Decoder::new(input)?;
    io::copy(&mut decoder, &mut output)?;
    Ok(())
}

/// Round-trips a byte stream using an in-memory fixture.
///
/// # Errors
///
/// Returns I/O and codec errors from either direction.
fn main() -> io::Result<()> {
    let input = b"Rust\0streams";
    let mut compressed = Vec::new();
    compress(&input[..], &mut compressed)?;
    let mut restored = Vec::new();
    decompress(compressed.as_slice(), &mut restored)?;
    assert_eq!(restored, input);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Handles an empty stream and binary input larger than the copy buffer.
    #[test]
    fn round_trips_empty_and_multi_chunk_input() -> io::Result<()> {
        let binary: Vec<u8> = (0..=u8::MAX).cycle().take(512 * 1024).collect();
        for input in [&[][..], binary.as_slice()] {
            let mut compressed = Vec::new();
            compress(input, &mut compressed)?;
            let mut decoded = Vec::new();
            decompress(compressed.as_slice(), &mut decoded)?;
            assert_eq!(decoded, input);
        }
        Ok(())
    }

    /// Reports a sink failure when finishing an empty frame.
    #[test]
    fn propagates_failure_while_finishing_frame() {
        /// Rejects all writes while allowing a flush.
        struct BrokenWriter;

        impl Write for BrokenWriter {
            fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
                Err(io::ErrorKind::BrokenPipe.into())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let error = compress(io::empty(), BrokenWriter).err();
        assert_eq!(
            error.map(|error| error.kind()),
            Some(io::ErrorKind::BrokenPipe)
        );
    }
}
