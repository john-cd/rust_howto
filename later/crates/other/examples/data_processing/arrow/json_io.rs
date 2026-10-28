// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Write an Arrow record batch to newline-delimited JSON and read it back.

use std::io::Cursor;
use std::sync::Arc;

use arrow::datatypes::Schema;
use arrow::error::ArrowError;
use arrow::error::Result;
use arrow::json;
use arrow::record_batch::RecordBatch;

/// Serializes a record batch to JSON, then reads the resulting bytes.
pub fn run(schema: Arc<Schema>, batch: &RecordBatch) -> Result<()> {
    let decoded = round_trip(schema, batch)?;
    assert_eq!(decoded, *batch);
    Ok(())
}

/// Writes a batch as line-delimited JSON and parses it with the supplied
/// schema.
fn round_trip(schema: Arc<Schema>, batch: &RecordBatch) -> Result<RecordBatch> {
    let mut json_bytes = Vec::new();
    let mut writer = json::LineDelimitedWriter::new(&mut json_bytes);
    writer.write(batch)?;
    writer.finish()?;

    let mut reader =
        json::ReaderBuilder::new(schema).build(Cursor::new(json_bytes))?;
    reader.next().transpose()?.ok_or_else(|| {
        ArrowError::ParseError("JSON input contained no rows".to_owned())
    })
}
// ANCHOR_END: example

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
