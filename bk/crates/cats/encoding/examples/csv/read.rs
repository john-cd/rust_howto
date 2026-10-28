// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example shows how to read a CSV file.

use csv::Error;

/// Parses the CSV input into string records, excluding the header row.
fn read_records(data: &str) -> Result<Vec<csv::StringRecord>, Error> {
    csv::Reader::from_reader(data.as_bytes())
        .records()
        .collect()
}

/// Reads CSV records and prints their fields.
fn main() -> Result<(), Error> {
    let data = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
    for record in read_records(data)? {
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
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

    /// Verifies the header is skipped and both data rows are parsed.
    #[test]
    fn reads_csv_records_and_skips_header() -> anyhow::Result<()> {
        let records = read_records(
            "year,make,model,description\n1948,Porsche,356,Luxury sports car\n1967,Ford,Mustang fastback 1967,American car",
        )?;
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].get(0), Some("1948"));
        assert_eq!(records[1].get(2), Some("Mustang fastback 1967"));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
