#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates invalid data handling when reading a CSV file.
use csv::Error;
use serde::Deserialize;

/// Represents a record with a name, place, and an optional ID.
///
/// The `id` field uses a custom deserialization function `csv::invalid_option`
/// to handle cases where the ID might not be a valid u64.
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

/// The main function that reads and deserializes CSV data.
fn main() -> Result<(), Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{record:?}");
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
