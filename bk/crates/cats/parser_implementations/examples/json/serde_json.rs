#![allow(dead_code)]
// ANCHOR: example
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use serde_json::Value;

/// Represents a user with a name, age, and a list of phone numbers.
#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u8,
    phones: Vec<String>,
}

/// Demonstrates parsing and serializing JSON data using the `serde_json` crate.
///
/// This function parses a JSON string into a `Value` enum and a custom `User`
/// struct, then serializes the `User` struct back into a JSON string.
fn main() -> Result<()> {
    let json_str = r#"
    {
        "name": "John Doe",
        "age": 42,
        "phones": [
            "+1 123456789",
            "+1 234567890"
        ]
    }
    "#;

    // 1. Parse the JSON string into a `Value` enum.
    // The `Value` enum can represent any valid JSON value.
    let parsed: Value = serde_json::from_str(json_str)?;
    // There are also `from_slice` for parsing from a byte slice `&[u8]`
    // and `from_reader` for parsing from any `io::Read` type,
    // such as a `File` or a TCP stream.

    // 2. Access parts of the data by indexing with square brackets.
    // The result of square bracket indexing is a borrow, so the type is
    // `&Value`.
    let name = parsed["name"].as_str().unwrap();
    let age = parsed["age"].as_i64().unwrap();
    let phones = parsed["phones"].as_array().unwrap();

    println!("Name: {name}");
    println!("Age: {age}");
    println!("Phones: {phones:?}");

    // 3. Parse the JSON string into a custom `User` struct.
    // The `User` struct must implement the `Deserialize` trait.
    let user: User = serde_json::from_str(json_str)?;
    println!("{user:?}");

    // 4. Serialize the `User` struct back into a JSON string.
    let u = serde_json::to_string(&user)?;
    println!("{u}");
    // There are also `serde_json::to_vec`, which serializes to a `Vec<u8>`,
    // and `serde_json::to_writer`, which serializes to any `io::Write`,
    // such as a `File` or a TCP stream.

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
