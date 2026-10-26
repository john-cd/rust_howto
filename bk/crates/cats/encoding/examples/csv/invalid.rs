#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates invalid data handling when reading a CSV file.

use csv::Error;
use serde::Deserialize;

/// Represents a record with a name, place, and an optional ID.
///
/// The `id` field uses a custom deserialization function `csv::invalid_option`
/// to handle cases where the ID might not be a valid u64.
#[derive(Debug, Deserialize, PartialEq)]
struct Record {
    /// The name.
    name: String,
    /// The place.
    place: String,
    /// The identifier, or `None` if it is not a valid number.
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

/// Deserializes CSV text while treating invalid `id` values as absent.
fn read_records(data: &str) -> Result<Vec<Record>, Error> {
    csv::Reader::from_reader(data.as_bytes())
        .deserialize()
        .collect()
}

/// Reads and prints CSV records with optional identifiers.
fn main() -> Result<(), Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";
    for record in read_records(data)? {
        println!("{record:?}");
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies invalid numeric fields become `None` and valid fields survive.
    #[test]
    fn maps_invalid_numeric_fields_to_none() -> anyhow::Result<()> {
        let records = read_records(
            "name,place,id\nmark,sydney,46.5\nashley,zurich,92\nakshat,delhi,37\nalisha,colombo,xyz",
        )?;
        assert_eq!(records.len(), 4);
        assert_eq!(records[0].id, None);
        assert_eq!(records[1].id, Some(92));
        assert_eq!(records[3].id, None);
        Ok(())
    }
}
