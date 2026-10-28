#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates how to use a custom delimiter during CSV deserialization.

use csv::Error;
use csv::ReaderBuilder;
use serde::Deserialize;

/// Represents a record to import.
#[derive(Debug, Deserialize, PartialEq)]
struct Record {
    /// The name.
    name: String,
    /// The place.
    place: String,
    /// The identifier, if any.
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

/// Parses tab-separated rows into typed records.
fn read_records(data: &str) -> Result<Vec<Record>, Error> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    reader.deserialize().collect()
}

/// Reads tab-separated data and prints each record.
fn main() -> Result<(), Error> {
    let data = "name\tplace\tid\nMark\tMelbourne\t46\nAshley\tZurich\t92";
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

    /// Verifies the configured tab delimiter separates fields into records.
    #[test]
    fn parses_tab_delimited_records() -> anyhow::Result<()> {
        let records = read_records(
            "name\tplace\tid\nMark\tMelbourne\t46\nAshley\tZurich\t92",
        )?;
        assert_eq!(
            records,
            vec![
                Record {
                    name: "Mark".to_owned(),
                    place: "Melbourne".to_owned(),
                    id: Some(46),
                },
                Record {
                    name: "Ashley".to_owned(),
                    place: "Zurich".to_owned(),
                    id: Some(92),
                },
            ]
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
