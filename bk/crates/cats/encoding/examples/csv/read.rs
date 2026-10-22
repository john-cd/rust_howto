#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example shows how to read a CSV file.

use csv::Error;

/// Reads CSV records and prints their fields.
fn main() -> Result<(), Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Error> {
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
