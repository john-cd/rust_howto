// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example shows how to filter CSV data based on a query.
//! It reads CSV data from a string, filters rows where any field matches the
//! query, and writes the filtered data to standard output.

use std::io;
use std::io::Write as _;

use anyhow::Result;

/// Returns the CSV header and rows containing the query.
fn filter_csv(data: &str, query: &str) -> Result<Vec<u8>> {
    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(Vec::new());
    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;
    Ok(wtr.into_inner()?)
}

/// Filters the inline CSV data and writes matching records to standard output.
fn main() -> Result<()> {
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";
    io::stdout().write_all(&filter_csv(data, "CA")?)?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies filtering preserves the header and only the matching row.
    #[test]
    fn preserves_header_and_filters_nonmatching_rows() -> anyhow::Result<()> {
        let data =
            "City,State,Population\nKenai,AK,7610\nWest Hollywood,CA,37031\n";
        let filtered = String::from_utf8(filter_csv(data, "CA")?)?;
        assert_eq!(
            filtered,
            "City,State,Population\nWest Hollywood,CA,37031\n"
        );
        Ok(())
    }
}
