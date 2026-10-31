//! Parses an Intel HEX data record and its end-of-file record.
//!
//! Intel HEX is a text format for firmware images: each line is a record with
//! an address, data bytes and a checksum. Programmers for microcontrollers and
//! EEPROM chips read it. An end-of-file record marks the last line.

// ANCHOR: example
use std::error::Error;

use ihex::Reader;
use ihex::Record;

/// A valid two-record Intel HEX object containing four bytes at offset zero.
const INTEL_HEX: &str = ":0400000001020304F2\n:00000001FF";

/// Parses all records in the sample Intel HEX object.
fn parse_records() -> Result<Vec<Record>, Box<dyn Error + Send + Sync>> {
    Reader::new(INTEL_HEX)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

/// Prints the number of records in the example Intel HEX object.
pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let records = parse_records()?;
    assert_eq!(
        records[0],
        Record::Data {
            offset: 0,
            value: vec![1, 2, 3, 4],
        }
    );
    println!("Parsed {} Intel HEX records.", records.len());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::Record;
    use super::parse_records;

    /// Verifies that the reader decodes the data and end-of-file records.
    #[test]
    fn parses_data_record() -> Result<(), Box<dyn Error + Send + Sync>> {
        let records = parse_records()?;

        assert_eq!(
            records,
            vec![
                Record::Data {
                    offset: 0,
                    value: vec![1, 2, 3, 4],
                },
                Record::EndOfFile,
            ]
        );
        Ok(())
    }
}
