#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build Arrow columns by hand with the `arrow-array` crate, combine them
//! into a `RecordBatch`, and print it with `arrow-cast`.

use std::sync::Arc;

use anyhow::Result;
use arrow_array::ArrayRef;
use arrow_array::Float64Array;
use arrow_array::Int32Array;
use arrow_array::RecordBatch;
use arrow_array::StringArray;
use arrow_cast::pretty::pretty_format_batches;

/// Builds a batch with an integer, a nullable float, and a string column.
///
/// `try_from_iter` infers each field's type from its array and marks a field
/// nullable only when the array contains nulls.
pub fn build_batch() -> Result<RecordBatch> {
    let ids = Int32Array::from(vec![1, 2, 3]);
    // `Option` elements become nulls in the validity bitmap.
    let prices = Float64Array::from(vec![Some(9.5), None, Some(12.25)]);
    let names = StringArray::from(vec!["apple", "banana", "cherry"]);

    let batch = RecordBatch::try_from_iter([
        ("id", Arc::new(ids) as ArrayRef),
        ("price", Arc::new(prices) as ArrayRef),
        ("name", Arc::new(names) as ArrayRef),
    ])?;
    Ok(batch)
}

/// Builds the batch and prints it as an ASCII table.
pub fn run() -> Result<RecordBatch> {
    let batch = build_batch()?;
    assert_eq!(batch.num_rows(), 3);
    assert_eq!(batch.num_columns(), 3);
    // Only the column that contains a null is nullable.
    assert!(!batch.schema().field(0).is_nullable());
    assert!(batch.schema().field(1).is_nullable());
    println!("{}", pretty_format_batches(std::slice::from_ref(&batch))?);
    Ok(batch)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use arrow_array::Array;

    use super::*;

    /// Verifies that the nullable float column keeps its null.
    #[test]
    fn nullable_column_keeps_its_null() -> Result<()> {
        let batch = run()?;
        assert_eq!(batch.column(1).null_count(), 1);
        assert!(batch.column(1).is_null(1));
        Ok(())
    }
}
