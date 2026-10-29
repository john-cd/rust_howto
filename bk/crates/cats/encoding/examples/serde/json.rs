// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates parsing a JSON string into a `serde_json::Value`
//! and comparing it to a `serde_json::Value` created using the `json!` macro.

use serde_json::Error;
use serde_json::Value;
use serde_json::json;

/// Parses a JSON string into a `serde_json::Value` and inspects it.
fn main() -> Result<(), Error> {
    let json_string: &str = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;
    // Parse the JSON string into a `serde_json::Value`.
    let parsed: Value = serde_json::from_str(json_string)?;

    let expected: Value = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });
    println!("{expected}");
    assert_eq!(parsed, expected);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the json example through its entry point.
    #[test]
    fn test_json_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Invalid JSON is an error; missing keys read as `Null`.
    #[test]
    fn parses_values_and_rejects_invalid_json() -> Result<(), Error> {
        assert!(serde_json::from_str::<Value>("{\"userid\": ").is_err());
        let value: Value = serde_json::from_str(r#"{"userid": 1}"#)?;
        assert_eq!(value["userid"], json!(1));
        assert_eq!(value["missing"], Value::Null);
        Ok(())
    }
}
