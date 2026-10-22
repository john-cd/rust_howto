#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates deserializing CSV records into a struct.

use serde::Deserialize;

/// Represents a record from the CSV data.
#[derive(Deserialize)]
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

/// Deserializes CSV records into a struct and prints them.
fn main() -> Result<(), csv::Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year, record.make, record.model, record.description
        );
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), csv::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
