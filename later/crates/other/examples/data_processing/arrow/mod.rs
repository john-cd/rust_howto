//! Runs six Apache Arrow examples: arrays, sub-crate arrays, IPC files, CSV,
//! JSON, and Serde conversion.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

/// Build typed Arrow arrays, a schema, and a record batch.
mod arrays;
/// Build Arrow columns by hand with the `arrow-array` crate, combine them
mod arrow_build;
/// Save a `RecordBatch` in the Arrow IPC file and stream formats with the
mod arrow_ipc_file;
/// Write an Arrow record batch to CSV and read it back.
mod csv_io;
/// Write an Arrow record batch to newline-delimited JSON and read it back.
mod json_io;
/// Convert Serde-compatible rows to an Arrow batch and back to Rust structs.
mod serde_convert;

/// Runs each Arrow example in sequence.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    let (schema, integer_values, float_values, batch) = arrays::run()?;
    arrays::sum_primitive_arrays(&integer_values, &float_values);
    let sub_crate_batch = arrow_build::run()?;
    arrow_ipc_file::run(&sub_crate_batch)?;
    csv_io::run(schema.clone(), &batch)?;
    json_io::run(schema, &batch)?;
    serde_convert::run()?;
    Ok(())
}

/// Verifies that all Arrow example modules run together.
#[cfg(test)]
mod tests {
    use super::run;

    /// Verifies that all Arrow example modules run together.
    #[test]
    fn arrow_examples_run_together() -> anyhow::Result<()> {
        run()
    }
}
