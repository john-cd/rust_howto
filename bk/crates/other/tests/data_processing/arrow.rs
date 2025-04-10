#![cfg(feature = "arrow")]
// ANCHOR: example
//! `arrow` is a complete, safe, native Rust implementation of `Apache Arrow`,
//! a cross-language development platform for in-memory data.
//!
//! For performance, it is recommended to override the LLVM compiler defaults
//! either by setting the RUSTFLAGS environment variable, or by setting
//! rustflags in your Cargo configuration. For example:
//!
//! ```sh
//! RUSTFLAGS="-C target-cpu=native -C target-feature=-prefer-256-bit"
//! ```

use std::sync::Arc;

use arrow::array::Array;
use arrow::array::Float64Array;
use arrow::array::Int32Array;
use arrow::array::StringArray;
use arrow::datatypes::DataType;
use arrow::datatypes::Field;
use arrow::datatypes::Schema;
use arrow::error::Result;
use arrow::record_batch::RecordBatch;
use arrow::util::pretty::print_batches;

/// This example demonstrates how to create and manipulate Arrow arrays and
/// record batches.
fn main() -> Result<()> {
    // Create a new `Int32Array`.
    // An `Int32Array` represents a (potentially nullable) array of `i32`
    // values.
    let int_array = Int32Array::from(vec![1, 2, 3, 4, 5]);

    // Check various properties of the array:
    assert_eq!(int_array.len(), 5);
    assert_eq!(int_array.value(0), 1);
    assert!(!int_array.is_null(0));

    // Create a new `Float64Array`.
    // A `Float64Array` represents a (potentially nullable) array of `f64`
    // values.
    let float_array = Float64Array::from(vec![5.0, 4.0, 3.0, 2.0, 1.0]);

    // Create a new `StringArray`.
    // A `StringArray` represents a (potentially nullable) array of UTF-8
    // encoded strings.
    let string_array = StringArray::from(vec!["a", "b", "c", "d", "e"]);

    // Create a schema for the data.
    // A `Schema` describes the fields (columns) of a record batch.
    let schema = Arc::new(Schema::new(vec![
        Field::new("integers", DataType::Int32, false),
        Field::new("floats", DataType::Float64, false),
        Field::new("strings", DataType::Utf8, false),
    ]));

    // Create a `RecordBatch` from the arrays and schema.
    // A `RecordBatch` groups one or more columns (arrays) together into a
    // tabular representation.
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(int_array) as Arc<dyn Array>,
            Arc::new(float_array.clone()) as Arc<dyn Array>,
            Arc::new(string_array) as Arc<dyn Array>,
        ],
    )?;

    // Pretty print the record batch:
    print_batches(&[batch])?;

    // Convert back to native types.
    // Here we iterate over the `Float64Array` and collect the values into a
    // `Vec`.
    let collected: Vec<_> = float_array.iter().collect();
    assert_eq!(
        collected,
        vec![Some(5.0), Some(4.0), Some(3.0), Some(2.0), Some(1.0)]
    );
    assert_eq!(float_array.values(), &[5.0, 4.0, 3.0, 2.0, 1.0]);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
// [add more; read / write from csv / JSON; serde; generic code; https://arrow.apache.org/rust/arrow/index.html](https://github.com/john-cd/rust_howto/issues/1081)
