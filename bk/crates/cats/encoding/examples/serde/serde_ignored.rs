// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `serde_ignored` is a crate that helps handle unknown fields when
//! deserializing with `serde`. It's useful to ignore fields
//! that are present in the input but not defined in your struct.
//!
//! This example demonstrates how to use `serde_ignored` to deserialize a JSON
//! object into a struct while ignoring any unknown fields.
//!
//! In `Cargo.toml`, add:
//! ```toml
//! [dependencies]
//! serde = { version = "1.0.217", features = ["derive"] } # or latest
//! serde_ignored = "0.1.10"
//! serde_json = "1.0.138"
//! ```

use serde::Deserialize;

/// A struct, whose JSON representation may contain unknown fields.
#[derive(Deserialize, Debug)]
struct MyStruct {
    /// A number.
    field1: i32,
    /// A string.
    field2: String,
}

/// Deserializes JSON and collects the fields that are ignored.
fn main() {
    let json_data = r#"
    {
        "field1": 42,
        "field2": "Hello, world!",
        "unknown_field": "This will be ignored",
        "another_unknown": 123
    }
    "#;

    // Create a vector to store ignored fields.
    let mut ignored_fields = Vec::new();

    // Deserialize the JSON data, collecting ignored fields.
    let result: Result<MyStruct, _> = serde_ignored::deserialize(
        &mut serde_json::Deserializer::from_str(json_data),
        |path: serde_ignored::Path| {
            println!("Ignored field found: {path}");
            ignored_fields.push(path.to_string());
        },
    );

    // Check the result of deserialization.
    match result {
        Ok(my_struct) => {
            println!("Deserialized struct: {my_struct:?}");
            println!("{}: {}", my_struct.field1, my_struct.field2);
            println!("All ignored fields: {ignored_fields:?}");
        }
        Err(e) => {
            println!("Error deserializing: {e}");
        }
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the serde ignored example through its entry point.
    #[test]
    fn test_serde_ignored_example_entry_point() {
        main();
    }

    /// Unknown fields are reported through the callback.
    #[test]
    fn reports_unknown_fields() -> Result<(), Box<dyn std::error::Error>> {
        let mut ignored = Vec::new();
        let parsed: MyStruct = serde_ignored::deserialize(
            &mut serde_json::Deserializer::from_str(
                r#"{"field1": 1, "field2": "a", "extra": true}"#,
            ),
            |path| ignored.push(path.to_string()),
        )?;
        assert_eq!(parsed.field1, 1);
        assert_eq!(parsed.field2, "a");
        assert_eq!(ignored, vec!["extra".to_owned()]);
        Ok(())
    }
}
