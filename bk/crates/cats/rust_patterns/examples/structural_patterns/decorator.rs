// ANCHOR: example
//! Wraps a reader to add byte-counting behavior.

use std::io::Cursor;
use std::io::Read;

/// A reader decorator that counts the bytes returned by its inner reader.
struct CountingReader<R> {
    inner: R,
    bytes_read: usize,
}

impl<R> CountingReader<R> {
    /// Wraps a reader and initializes its byte count to zero.
    fn new(inner: R) -> Self {
        Self {
            inner,
            bytes_read: 0,
        }
    }

    /// Returns the total number of bytes returned by successful reads.
    fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}

impl<R: Read> Read for CountingReader<R> {
    /// Reads from the inner reader and records the number of bytes returned.
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.inner.read(buffer)?;
        self.bytes_read += bytes_read;
        Ok(bytes_read)
    }
}

/// Reads text through the decorator and reports the count.
fn main() -> anyhow::Result<()> {
    let mut reader = CountingReader::new(Cursor::new(b"Rust"));
    let mut text = String::new();
    reader.read_to_string(&mut text)?;

    println!("Read {text:?} ({} bytes).", reader.bytes_read());
    Ok(())
}
// ANCHOR_END: example

/// Runs the reader decorator example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::Read;

    use super::CountingReader;

    /// Verifies the decorator preserves the input and tracks bytes returned.
    #[test]
    fn counts_bytes_read() {
        let mut reader = CountingReader::new(Cursor::new(b"Rust"));
        let mut text = String::new();
        reader
            .read_to_string(&mut text)
            .expect("the in-memory input should be readable");

        assert_eq!(text, "Rust");
        assert_eq!(reader.bytes_read(), 4);
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
