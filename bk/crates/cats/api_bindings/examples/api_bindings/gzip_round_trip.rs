// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Use `flate2`'s safe Rust API to write and read gzip data.

use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

/// Compresses and restores a short message through the gzip API.
fn example() -> std::io::Result<()> {
    let message = b"Safe wrappers make native and format APIs feel like Rust.";

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(message)?;
    let compressed = encoder.finish()?;

    let mut decoder = GzDecoder::new(Cursor::new(&compressed));
    let mut restored = Vec::new();
    decoder.read_to_end(&mut restored)?;

    assert_eq!(restored, message);
    println!(
        "Compressed {} bytes into {} bytes",
        message.len(),
        compressed.len()
    );
    Ok(())
}

/// Runs the gzip binding example and reports I/O failures to the caller.
fn main() -> std::io::Result<()> {
    example()
}
// ANCHOR_END: example

/// Runs the example logic exposed by this module.
pub fn run() -> std::io::Result<()> {
    main()
}

/// Provides entry point tests behavior for the gzip round trip component.
#[cfg(test)]
mod entry_point_tests {
    use super::*;

    /// Runs the example entry point and propagates I/O failures.
    #[test]
    fn private_main_runs() -> std::io::Result<()> {
        main()
    }

    /// Runs the example function and propagates I/O failures.
    #[test]
    fn example_runs() -> std::io::Result<()> {
        example()
    }
}
