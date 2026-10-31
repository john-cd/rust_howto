// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Build typed Arrow arrays, a schema, and a record batch.
//!
//! Apache Arrow is a columnar in-memory format: each column of a table is
//! stored contiguously, which makes scans and analytics fast and lets different
//! tools (Python, Rust, databases) share data without copying. A record batch
//! is a set of equally long columns plus a schema.

use std::sync::Arc;

use arrow::array::Array;
use arrow::array::Float64Array;
use arrow::array::Int32Array;
use arrow::array::PrimitiveArray;
use arrow::array::StringArray;
use arrow::datatypes::ArrowPrimitiveType;
use arrow::datatypes::DataType;
use arrow::datatypes::Field;
use arrow::datatypes::Schema;
use arrow::error::Result;
use arrow::record_batch::RecordBatch;
use arrow::util::pretty::print_batches;

/// Creates the sample typed Arrow arrays.
fn create_arrays() -> (Int32Array, Float64Array, StringArray) {
    let integers = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let floats = Float64Array::from(vec![5.0, 4.0, 3.0, 2.0, 1.0]);
    let strings = StringArray::from(vec!["a", "b", "c", "d", "e"]);
    (integers, floats, strings)
}

/// Creates the Arrow schema describing the sample arrays.
fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("integers", DataType::Int32, false),
        Field::new("floats", DataType::Float64, false),
        Field::new("strings", DataType::Utf8, false),
    ]))
}

/// Combines the sample arrays into a `RecordBatch` according to the schema.
fn create_record_batch(
    schema: Arc<Schema>,
    integers: &Int32Array,
    floats: &Float64Array,
    strings: &StringArray,
) -> Result<RecordBatch> {
    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(integers.clone()) as Arc<dyn Array>,
            Arc::new(floats.clone()) as Arc<dyn Array>,
            Arc::new(strings.clone()) as Arc<dyn Array>,
        ],
    )
}

/// Builds the sample arrays and prints their record batch.
pub fn run() -> Result<(Arc<Schema>, Int32Array, Float64Array, RecordBatch)> {
    let (integers, floats, strings) = create_arrays();
    let schema = create_schema();
    let batch =
        create_record_batch(schema.clone(), &integers, &floats, &strings)?;

    assert_eq!(integers.len(), 5);
    assert_eq!(integers.value(0), 1);
    assert!(!integers.is_null(0));
    assert_eq!(floats.values(), &[5.0, 4.0, 3.0, 2.0, 1.0]);
    print_batches(std::slice::from_ref(&batch))?;

    Ok((schema, integers, floats, batch))
}

/// Sums nullable primitive arrays, treating null elements as zero.
fn sum<T: ArrowPrimitiveType>(array: &PrimitiveArray<T>) -> T::Native
where
    T::Native: std::iter::Sum + Default,
{
    array
        .iter()
        .map(std::option::Option::unwrap_or_default)
        .sum()
}

/// Demonstrates that the same generic sum works with integer and float arrays.
pub fn sum_primitive_arrays(integers: &Int32Array, floats: &Float64Array) {
    assert_eq!(sum(integers), 15);
    assert_eq!(sum(floats), 15.0);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the schema, arrays, and values in the sample record batch.
    #[test]
    fn arrays_form_a_typed_record_batch() -> Result<()> {
        let (schema, integers, floats, batch) = run()?;

        assert_eq!(schema.fields().len(), 3);
        assert_eq!(batch.num_rows(), 5);
        sum_primitive_arrays(&integers, &floats);
        Ok(())
    }
}
