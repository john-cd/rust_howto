//! Runs six Apache Arrow examples: arrays, sub-crate arrays, IPC files, CSV,
//! JSON, and Serde conversion.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Subcommand;

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

/// The Arrow recipes available from the command line.
#[derive(Subcommand)]
pub enum Recipe {
    /// Runs all Arrow recipes in order.
    All,
    /// Builds typed arrays and sums their values.
    Arrays,
    /// Builds and prints a batch using the Arrow sub-crates.
    ArrowBuild,
    /// Round-trips a record batch through Arrow IPC.
    Ipc,
    /// Round-trips a record batch through CSV.
    Csv,
    /// Round-trips a record batch through newline-delimited JSON.
    Json,
    /// Round-trips Rust structs through an Arrow batch.
    Serde,
}

/// Runs one Arrow recipe and builds the input it needs.
///
/// # Errors
///
/// Returns any failure from the selected example.
pub fn run(recipe: Recipe) -> Result<()> {
    match recipe {
        Recipe::All => run_all(),
        Recipe::Arrays => {
            let (_, integers, floats, _) = arrays::run()?;
            arrays::sum_primitive_arrays(&integers, &floats);
            Ok(())
        }
        Recipe::ArrowBuild => arrow_build::run().map(drop),
        Recipe::Ipc => arrow_ipc_file::run(&arrow_build::run()?),
        Recipe::Csv => {
            let (schema, _, _, batch) = arrays::run()?;
            csv_io::run(schema, &batch).map_err(Into::into)
        }
        Recipe::Json => {
            let (schema, _, _, batch) = arrays::run()?;
            json_io::run(schema, &batch).map_err(Into::into)
        }
        Recipe::Serde => serde_convert::run(),
    }
}

/// Runs each Arrow example in sequence.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
fn run_all() -> Result<()> {
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
    use super::Recipe;
    use super::run;

    /// Verifies that all Arrow example modules run together.
    #[test]
    fn arrow_examples_run_together() -> anyhow::Result<()> {
        run(Recipe::All)
    }
}
