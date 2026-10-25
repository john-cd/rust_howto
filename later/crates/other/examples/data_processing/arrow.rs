#![allow(dead_code)]
// TODO Reviewed ??-??-2026

//! Runs four Apache Arrow examples: arrays, CSV, JSON, and Serde conversion.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

mod arrays;
mod csv_io;
mod json_io;
mod serde_convert;

/// Runs each Arrow example in sequence.
pub fn run() -> Result<()> {
    let (schema, integer_values, float_values, batch) = arrays::run()?;
    arrays::sum_primitive_arrays(&integer_values, &float_values);
    csv_io::run(schema.clone(), &batch)?;
    json_io::run(schema, &batch)?;
    serde_convert::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    /// Verifies that all four Arrow example modules run together.
    #[test]
    fn arrow_examples_run_together() -> anyhow::Result<()> {
        run()
    }
}
