// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Write an Arrow record batch to CSV and read it back.
//!
//! CSV is a plain-text format with no types, while Arrow columns are typed.
//! Converting between them needs a schema, or inference from the data.

use std::io::Cursor;
use std::sync::Arc;

use arrow::csv;
use arrow::datatypes::Schema;
use arrow::error::ArrowError;
use arrow::error::Result;
use arrow::record_batch::RecordBatch;

/// Serializes a record batch to CSV, then reads the resulting bytes.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn example_body(schema: Arc<Schema>, batch: &RecordBatch) -> Result<()> {
    let decoded = round_trip(schema, batch)?;
    assert_eq!(decoded, *batch);
    Ok(())
}

/// Writes a batch with a header and parses it using the supplied schema.
fn round_trip(schema: Arc<Schema>, batch: &RecordBatch) -> Result<RecordBatch> {
    let mut csv_bytes = Vec::new();
    csv::Writer::new(&mut csv_bytes).write(batch)?;

    let mut reader = csv::ReaderBuilder::new(schema)
        .with_header(true)
        .build(Cursor::new(csv_bytes))?;
    reader
        .next()
        .transpose()?
        .ok_or_else(|| ArrowError::ParseError("CSV input contained no rows".to_owned()))
}
// ANCHOR_END: example

/// Writes the sample record batch as CSV.
///
/// # Arguments
///
/// * `schema` - Schema used to decode the CSV columns.
/// * `batch` - Record batch to serialize and read back.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run(schema: Arc<Schema>, batch: &RecordBatch) -> Result<()> {
    example_body(schema, batch)
}

/// Verifies that CSV output preserves all record batch values.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrow::arrays;

    /// Verifies that CSV output preserves all record batch values.
    #[test]
    fn csv_round_trip_preserves_the_record_batch() -> Result<()> {
        let (schema, _, _, batch) = arrays::run()?;
        run(schema, &batch)
    }
}
