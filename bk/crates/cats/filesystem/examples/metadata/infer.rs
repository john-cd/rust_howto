#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Detect a file's type from its first bytes.

use std::fs;
use std::path::Path;

/// The first bytes of a PNG file.
const PNG_HEADER: &[u8] = b"\x89PNG\r\n\x1a\n";

/// The first bytes of a ZIP archive.
const ZIP_HEADER: &[u8] = b"PK\x03\x04";

/// Returns the MIME type detected from the content, if it is recognized.
fn detect(bytes: &[u8]) -> Option<&'static str> {
    infer::get(bytes).map(|kind| kind.mime_type())
}

/// Returns the MIME type detected from the content of the file at `path`.
fn detect_file(path: &Path) -> std::io::Result<Option<&'static str>> {
    Ok(infer::get_from_path(path)?.map(|kind| kind.mime_type()))
}

/// Shows that the extension of a file can lie.
pub fn run() -> anyhow::Result<()> {
    assert_eq!(detect(PNG_HEADER), Some("image/png"));
    assert_eq!(detect(ZIP_HEADER), Some("application/zip"));
    // Plain text has no magic number.
    assert_eq!(detect(b"just some text"), None);

    // A ZIP archive named like an image: the content wins.
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("holiday.png");
    fs::write(&path, ZIP_HEADER)?;
    println!("{} is {:?}", path.display(), detect_file(&path)?);
    assert_eq!(detect_file(&path)?, Some("application/zip"));

    // `infer::is_image` and similar helpers test a whole family of types.
    assert!(infer::is_image(PNG_HEADER));
    assert!(!infer::is_image(ZIP_HEADER));
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
