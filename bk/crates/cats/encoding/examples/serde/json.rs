#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates parsing a JSON string into a `serde_json::Value`
//! and comparing it to a `serde_json::Value` created using the `json!` macro.

use serde_json::Error;
use serde_json::Value;
use serde_json::json;

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
    println!("{}", expected);
    assert_eq!(parsed, expected);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
