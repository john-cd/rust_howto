// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Derive a JSON Schema with [`schemars`] and validate data against it with
//! [`jsonschema`].
//!
//! A JSON Schema describes the shape of JSON data: its fields, types and
//! constraints. It can be derived from Rust types and used to validate data,
//! generate documentation or feed editors with autocompletion.

use schemars::JsonSchema;
use schemars::schema_for;
use serde::Deserialize;

/// Application configuration.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct Config {
    /// The port to listen on.
    pub port: u16,

    /// Enables verbose logging.
    #[serde(default)]
    pub verbose: bool,
}

/// The JSON Schema of `Config`.
///
/// # Errors
///
/// Returns an error if the generated schema cannot be serialized as JSON.
pub fn config_schema() -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(schema_for!(Config))?)
}

/// Validates `instance` against the schema.
///
/// # Errors
///
/// Returns an error if schema serialization or validator construction fails.
pub fn is_valid(instance: &serde_json::Value) -> anyhow::Result<bool> {
    let validator = jsonschema::validator_for(&config_schema()?)?;
    Ok(validator.is_valid(instance))
}

/// Prints the generated schema and demonstrates accepting and rejecting data.
fn main() -> anyhow::Result<()> {
    let schema = config_schema()?;
    println!("{schema}");
    let valid = is_valid(&serde_json::json!({ "port": 8080 }))?;
    let invalid = is_valid(&serde_json::json!({ "port": "text" }))?;
    println!("valid: {valid}, invalid: {invalid}");
    let config: Config = serde_json::from_value(serde_json::json!({ "port": 8080 }))?;
    println!("port: {}, verbose: {}", config.port, config.verbose);
    Ok(())
}
// ANCHOR_END: example

/// Generates the configuration schema shown in the example.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    let entry_point = main;
    entry_point()
}

/// Accepts valid configuration and rejects wrong types or missing fields.
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// Runs the schema example through its private entry point.
    #[test]
    fn private_main_runs() -> anyhow::Result<()> {
        main()
    }

    /// Accepts valid configuration and rejects wrong types or missing fields.
    #[test]
    fn round_trip() -> anyhow::Result<()> {
        assert!(is_valid(&json!({ "port": 8080 }))?);
        assert!(!is_valid(&json!({ "port": "eighty" }))?);
        assert!(!is_valid(&json!({ "verbose": true }))?);
        Ok(())
    }

    /// Includes the configuration fields in the generated schema.
    #[test]
    fn schema_lists_properties() -> anyhow::Result<()> {
        let schema = config_schema()?;
        assert!(schema["properties"]["port"].is_object());
        Ok(())
    }
}
