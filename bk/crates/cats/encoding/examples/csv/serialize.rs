// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example shows how to serialize data to a CSV file.
//!
//! CSV (comma-separated values) is a plain-text table format that spreadsheets
//! open directly. The writer takes care of quoting fields that contain commas,
//! quotes or line breaks.

use std::io;
use std::io::Write as _;

use anyhow::Result;

/// Serializes a header and records into CSV bytes.
fn serialize_records() -> Result<Vec<u8>> {
    let mut wtr = csv::Writer::from_writer(Vec::new());

    wtr.write_record(["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(wtr.into_inner()?)
}

/// Writes the serialized records to standard output.
fn main() -> Result<()> {
    io::stdout().write_all(&serialize_records()?)?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the serialized header and tuple fields are written in order.
    #[test]
    fn serializes_tuple_records_and_header() -> anyhow::Result<()> {
        let output = String::from_utf8(serialize_records()?)?;
        assert_eq!(
            output,
            "Name,Place,ID\nMark,Sydney,87\nAshley,Dublin,32\nAkshat,Delhi,11\n"
        );
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
