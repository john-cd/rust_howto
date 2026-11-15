// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Save a `RecordBatch` in the Arrow IPC file and stream formats with the
//! `arrow-ipc` crate, then read it back.
//!
//! The Arrow IPC (inter-process communication) format saves record batches
//! exactly as they are in memory, so other programs can read them with almost
//! no parsing. The file format supports random access; the stream format suits
//! pipes and sockets.

use std::io::Cursor;

use anyhow::Context;
use anyhow::Result;
use arrow_array::RecordBatch;
use arrow_ipc::reader::FileReader;
use arrow_ipc::reader::StreamReader;
use arrow_ipc::writer::FileWriter;
use arrow_ipc::writer::StreamWriter;

/// Encodes a batch in the IPC *file* format (random access, footer).
///
/// # Errors
/// Returns an error if Arrow cannot create, serialize, or finalize the IPC file.
pub fn write_file(batch: &RecordBatch) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut writer = FileWriter::try_new(&mut buffer, &batch.schema())?;
    writer.write(batch)?;
    // `finish` writes the footer; without it the file cannot be read.
    writer.finish()?;
    drop(writer);
    Ok(buffer)
}

/// Decodes every batch of an IPC file held in memory.

/// # Errors
/// Returns an error if `bytes` is not a valid IPC file or a batch cannot be decoded.
pub fn read_file(bytes: Vec<u8>) -> Result<Vec<RecordBatch>> {
    let reader = FileReader::try_new(Cursor::new(bytes), None)?;
    reader
        .collect::<Result<Vec<_>, _>>()
        .context("invalid Arrow IPC file")
}

/// Encodes a batch in the IPC *stream* format, suited to sockets and pipes.

/// # Errors
/// Returns an error if Arrow cannot create, serialize, or finalize the IPC stream.
pub fn write_stream(batch: &RecordBatch) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut writer = StreamWriter::try_new(&mut buffer, &batch.schema())?;
    writer.write(batch)?;
    writer.finish()?;
    drop(writer);
    Ok(buffer)
}

/// Decodes every batch of an IPC stream held in memory.

/// # Errors
/// Returns an error if `bytes` is not a valid IPC stream or a batch cannot be decoded.
pub fn read_stream(bytes: Vec<u8>) -> Result<Vec<RecordBatch>> {
    let reader = StreamReader::try_new(Cursor::new(bytes), None)?;
    reader
        .collect::<Result<Vec<_>, _>>()
        .context("invalid Arrow IPC stream")
}

/// Round-trips a batch through both IPC formats.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn example_body(batch: &RecordBatch) -> Result<()> {
    let from_file = read_file(write_file(batch)?)?;
    assert_eq!(from_file, vec![batch.clone()]);

    let from_stream = read_stream(write_stream(batch)?)?;
    assert_eq!(from_stream, vec![batch.clone()]);
    Ok(())
}
// ANCHOR_END: example

/// Writes and reads back the sample record batch as an Arrow IPC file.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> Result<()> {
    let (_, _, _, batch) = arrays::run()?;
    example_body(&batch)
}

/// Verifies that schema, nulls, and values survive an IPC file round trip.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrow::arrow_build;

    /// Verifies that schema, nulls, and values survive an IPC file round trip.
    #[test]
    fn ipc_file_round_trip_preserves_the_batch() -> Result<()> {
        let batch = arrow_build::build_batch()?;
        let decoded = read_file(write_file(&batch)?)?;
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].schema(), batch.schema());
        assert_eq!(decoded[0], batch);
        Ok(())
    }

    /// Verifies that the stream format round-trips the same batch.
    #[test]
    fn ipc_stream_round_trip_preserves_the_batch() -> Result<()> {
        run(&arrow_build::build_batch()?)
    }

    #[test]
    fn invalid_bytes_are_an_error() {
        assert!(read_file(b"not arrow".to_vec()).is_err());
    }
}
