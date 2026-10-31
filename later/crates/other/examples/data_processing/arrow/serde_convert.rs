// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Convert Serde-compatible rows to an Arrow batch and back to Rust structs.
//!
//! Serde can convert Rust structs to Arrow columns and back, which saves
//! writing the column code by hand. It is convenient but slower than building
//! the columns directly for large data.

use std::sync::Arc;

use anyhow::Result;
use arrow::datatypes::DataType;
use arrow::datatypes::Field;
use arrow::datatypes::Schema;
use arrow::json;
use serde::Deserialize;
use serde::Serialize;

/// A Rust row that can be serialized into and deserialized from Arrow JSON.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Person {
    /// The person's name.
    name: String,
    /// The person's age in years.
    age: i32,
}

/// Converts rows to Arrow, writes Arrow as JSON, and converts the JSON to rows.
pub fn run() -> Result<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("age", DataType::Int32, false),
    ]));
    let people = vec![
        Person {
            name: "Alice".to_owned(),
            age: 30,
        },
        Person {
            name: "Bob".to_owned(),
            age: 41,
        },
    ];

    let mut decoder = json::ReaderBuilder::new(schema).build_decoder()?;
    decoder.serialize(&people)?;
    let batch = decoder.flush()?.ok_or_else(|| {
        arrow::error::ArrowError::ParseError("no rows to convert".to_owned())
    })?;

    let mut writer = json::ArrayWriter::new(Vec::new());
    writer.write(&batch)?;
    writer.finish()?;
    let decoded_people: Vec<Person> =
        serde_json::from_slice(&writer.into_inner())?;
    assert_eq!(decoded_people, people);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::run;

    /// Verifies that Serde-compatible rows round-trip through Arrow.
    #[test]
    fn serde_rows_round_trip_through_arrow() -> anyhow::Result<()> {
        run()
    }
}
