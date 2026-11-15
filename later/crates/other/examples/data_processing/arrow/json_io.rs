// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Write an Arrow record batch to newline-delimited JSON and read it back.
//!
//! Newline-delimited JSON (NDJSON) holds one JSON object per line, so files can
//! be streamed and split. It is common for logs and data exports.

use std::io::Cursor;
use std::sync::Arc;

use arrow::datatypes::Schema;
use arrow::error::ArrowError;
use arrow::error::Result;
use arrow::json;
use arrow::record_batch::RecordBatch;

/// Serializes a record batch to JSON, then reads the resulting bytes.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn example_body(schema: Arc<Schema>, batch: &RecordBatch) -> Result<()> {
    let decoded = round_trip(schema, batch)?;
    assert_eq!(decoded, *batch);
    Ok(())
}
/// Writes `batch` as line-delimited JSON and reads the bytes using `schema`.
///
/// # Errors
/// Returns an error if Arrow cannot serialize the batch or parse the JSON back
/// into a record batch with `schema`.
fn round_trip(schema: Arc<Schema>, batch: &RecordBatch) -> Result<RecordBatch> {
    let mut json_bytes = Vec::new();
    let mut writer = json::LineDelimitedWriter::new(&mut json_bytes);
    writer.write(batch)?;
    writer.finish()?;

    let mut reader = json::ReaderBuilder::new(schema).build(Cursor::new(json_bytes))?;
    reader
        .next()
        .transpose()?
        .ok_or_else(|| ArrowError::ParseError("JSON input contained no rows".to_owned()))
}
// ANCHOR_END: example

/// Writes the sample record batch as JSON.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> Result<()> {
    let (schema, _, _, batch) = arrays::run()?;
    example_body(schema, &batch)
}

/// Verifies that JSON output preserves all record batch values.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrow::arrays;

    /// Verifies that JSON output preserves all record batch values.
    #[test]
    fn json_round_trip_preserves_the_record_batch() -> Result<()> {
        let (schema, _, _, batch) = arrays::run()?;
        run(schema, &batch)
    }
}
