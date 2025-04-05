// ANCHOR: example
//! `serde_ignored` is a crate that helps you handle unknown fields when
//! deserializing with `serde`. It's useful when you want to ignore fields
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MyStruct {
    field1: i32,
    field2: String,
}

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
            println!("Ignored field found: {}", path);
            ignored_fields.push(path.to_string());
        },
    );

    // Check the result of deserialization.
    match result {
        Ok(my_struct) => {
            println!("Deserialized struct: {:?}", my_struct);
            println!("All ignored fields: {:?}", ignored_fields);
        }
        Err(e) => {
            println!("Error deserializing: {}", e);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
