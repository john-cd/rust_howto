// TODO Reviewed ??-??-2026.
//! Compares input-backed JSON fields with owned fields read from a stream.

// ANCHOR: example
use std::borrow::Cow;
use std::io::BufReader;
use std::io::Read;

use serde::Deserialize;
use serde::de::DeserializeOwned;

/// A record whose name may share storage with the input.
#[derive(Debug, Deserialize)]
struct BorrowedRecord<'a> {
    /// Borrows unescaped text and owns text that must be unescaped.
    #[serde(borrow)]
    name: Cow<'a, str>,
}

/// A record that owns its name.
#[derive(Debug, Deserialize)]
struct OwnedRecord {
    /// Holds the decoded name independently of the reader.
    name: String,
}

/// Parses a JSON buffer that remains available for the lifetime of the result.
///
/// # Errors
///
/// Returns a JSON error if input cannot be decoded as the requested type.
fn from_slice<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> serde_json::Result<T> {
    serde_json::from_slice(bytes)
}

/// Parses one complete JSON value from a buffered reader.
///
/// # Errors
///
/// Returns an error if reading fails or the value cannot be decoded.
fn from_reader<T: DeserializeOwned>(reader: impl Read) -> serde_json::Result<T> {
    serde_json::from_reader(BufReader::new(reader))
}

/// Parses the same record from a slice and from a reader.
///
/// # Errors
///
/// Returns a JSON decoding or I/O error if an input cannot be read as a record.
fn main() -> serde_json::Result<()> {
    let input = br#"{"name":"Ada"}"#;
    let borrowed: BorrowedRecord<'_> = from_slice(input)?;
    let owned: OwnedRecord = from_reader(&input[..])?;
    println!("{} == {}", borrowed.name, owned.name);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Borrows plain text and allocates only when escape sequences must be decoded.
    #[test]
    fn borrows_only_when_json_contains_the_decoded_text() -> serde_json::Result<()> {
        let plain: BorrowedRecord<'_> = from_slice(br#"{"name":"Ada"}"#)?;
        let escaped: BorrowedRecord<'_> = from_slice(br#"{"name":"\u0041da"}"#)?;
        assert!(matches!(plain.name, Cow::Borrowed("Ada")));
        assert!(matches!(escaped.name, Cow::Owned(ref text) if text == "Ada"));
        Ok(())
    }

    /// Reads both records into owned strings that survive the input buffer.
    #[test]
    fn reader_produces_owned_records() -> serde_json::Result<()> {
        for source in [r#"{"name":"Ada"}"#, r#"{"name":"\u0041da"}"#] {
            let bytes = source.as_bytes().to_vec();
            let record: OwnedRecord = from_reader(bytes.as_slice())?;
            drop(bytes);
            assert_eq!(record.name, "Ada");
        }
        Ok(())
    }
}
