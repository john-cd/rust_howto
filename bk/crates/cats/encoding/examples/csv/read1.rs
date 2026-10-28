// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates deserializing CSV records into a struct.

use serde::Deserialize;

/// Represents a record from the CSV data.
#[derive(Debug, Deserialize, PartialEq)]
struct Record {
    /// The year the car was built.
    year: u16,
    /// The manufacturer.
    make: String,
    /// The model.
    model: String,
    /// A description of the car.
    description: String,
}

/// Deserializes CSV records from text into typed rows.
fn read_records(data: &str) -> Result<Vec<Record>, csv::Error> {
    csv::Reader::from_reader(data.as_bytes())
        .deserialize()
        .collect()
}

/// Deserializes CSV records into a struct and prints them.
fn main() -> Result<(), csv::Error> {
    let data = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
    for record in read_records(data)? {
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year, record.make, record.model, record.description
        );
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), csv::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that CSV columns deserialize into the corresponding fields.
    #[test]
    fn deserializes_csv_records_into_typed_rows() -> anyhow::Result<()> {
        let records = read_records(
            "year,make,model,description\n1948,Porsche,356,Luxury sports car\n1967,Ford,Mustang fastback 1967,American car",
        )?;
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].year, 1948);
        assert_eq!(records[0].make, "Porsche");
        assert_eq!(records[1].description, "American car");
        Ok(())
    }
}
