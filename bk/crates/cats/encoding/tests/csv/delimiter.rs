#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to use a custom delimiter during CSV deserialization.

use csv::Error;
use csv::ReaderBuilder;
use serde::Deserialize;

/// Represents a record to import.
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Error> {
    // Tab-separated data.
    let tab_separated_data = "name\tplace\tid
  Mark\tMelbourne\t46
  Ashley\tZurich\t92";

    // Create a CSV reader with a tab delimiter.
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(tab_separated_data.as_bytes());

    // Iterate over the records, deserializing each into a `Record` struct.
    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
