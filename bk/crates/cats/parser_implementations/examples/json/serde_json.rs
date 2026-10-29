// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates parsing and serializing JSON data using the `serde_json` crate.

use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use serde_json::Value;

/// Represents a user with a name, age, and a list of phone numbers.
#[derive(Deserialize, Serialize, Debug)]
struct User {
    /// The name of the user.
    name: String,
    /// The age of the user.
    age: u8,
    /// The phone numbers of the user.
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
    let name = parsed["name"].as_str().expect("name should be a string");
    let age = parsed["age"].as_i64().expect("age should be an integer");
    let phones = parsed["phones"]
        .as_array()
        .expect("phones should be an array");

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

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the serde json example through its entry point.
    #[test]
    fn test_serde_json_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    const SAMPLE: &str =
        r#"{"name": "John Doe", "age": 42, "phones": ["+1 123", "+1 234"]}"#;

    /// The typed and untyped parses agree.
    #[test]
    fn typed_and_untyped_parsing_agree() -> Result<()> {
        let value: Value = serde_json::from_str(SAMPLE)?;
        let user: User = serde_json::from_str(SAMPLE)?;
        assert_eq!(value["name"], user.name.as_str());
        assert_eq!(user.age, 42);
        assert_eq!(user.phones, ["+1 123", "+1 234"]);
        Ok(())
    }

    /// Serialization round-trips.
    #[test]
    fn serialization_round_trips() -> Result<()> {
        let user: User = serde_json::from_str(SAMPLE)?;
        let text = serde_json::to_string(&user)?;
        assert_eq!(
            serde_json::from_str::<Value>(&text)?,
            serde_json::from_str::<Value>(SAMPLE)?
        );
        Ok(())
    }

    /// A value out of range for `u8` is rejected.
    #[test]
    fn out_of_range_age_is_rejected() {
        let text = r#"{"name": "x", "age": 300, "phones": []}"#;
        assert!(serde_json::from_str::<User>(text).is_err());
    }

    /// Truncated input is an error.
    #[test]
    fn truncated_input_is_rejected() {
        assert!(serde_json::from_str::<Value>(r#"{"name": "#).is_err());
    }
}
