// TODO Reviewed ??-??-2026.

//! Runs six Apache Arrow examples: arrays, sub-crate arrays, IPC files, CSV,
//! JSON, and Serde conversion.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

mod arrays;
mod arrow_build;
mod arrow_ipc_file;
mod csv_io;
mod json_io;
mod serde_convert;

/// Runs each Arrow example in sequence.
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

#[cfg(test)]
mod tests {
    use super::run;

    /// Verifies that all Arrow example modules run together.
    #[test]
    fn arrow_examples_run_together() -> anyhow::Result<()> {
        run()
    }
}
