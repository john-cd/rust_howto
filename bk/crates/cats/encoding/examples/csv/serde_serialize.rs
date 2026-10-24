#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates serializing structs to CSV using `serde`.

use std::io;
use std::io::Write as _;

use anyhow::Result;
use serde::Serialize;

/// A simple struct representing a record with a name, place, and ID.
#[derive(Serialize)]
struct Record<'a> {
    /// The name.
    name: &'a str,
    /// The place.
    place: &'a str,
    /// The identifier.
    id: u64,
}

/// Serializes sample records into CSV bytes with the header inferred by Serde.
fn serialize_records() -> Result<Vec<u8>> {
    let mut wtr = csv::Writer::from_writer(Vec::new());

    let rec1 = Record {
        name: "Mark",
        place: "Melbourne",
        id: 56,
    };
    let rec2 = Record {
        name: "Ashley",
        place: "Sydney",
        id: 64,
    };
    let rec3 = Record {
        name: "Akshat",
        place: "Delhi",
        id: 98,
    };

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;
    Ok(wtr.into_inner()?)
}

/// Writes the serialized records to standard output.
fn main() -> Result<()> {
    io::stdout().write_all(&serialize_records()?)?;
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies Serde writes struct field names and values as CSV columns.
    #[test]
    fn serializes_struct_fields_as_csv_columns() -> anyhow::Result<()> {
        let output = String::from_utf8(serialize_records()?)?;
        assert_eq!(
            output,
            "name,place,id\nMark,Melbourne,56\nAshley,Sydney,64\nAkshat,Delhi,98\n"
        );
        Ok(())
    }
}
