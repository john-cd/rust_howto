//! Reads a Windows-1252 fixture as UTF-8 text, one line at a time.

// ANCHOR: example
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

/// Opens the fixture, decodes its lines, and checks the resulting text.
///
/// # Errors
///
/// Returns an I/O error if the fixture cannot be opened or read.
fn main() -> std::io::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/fixtures/windows-1252.txt");
    let file = File::open(path)?;
    let decoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut lines = BufReader::new(decoded).lines();

    assert_eq!(lines.next().transpose()?.as_deref(), Some("Café"));
    assert_eq!(lines.next().transpose()?.as_deref(), Some("Price: €5"));
    assert!(lines.next().is_none());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks accented letters and the euro sign are decoded from the fixture.
    #[test]
    fn reads_windows_1252_as_utf8() -> std::io::Result<()> {
        super::main()
    }
}
