#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Derive a JSON Schema with [`schemars`] and validate data against it with
//! [`jsonschema`].

use schemars::JsonSchema;
use schemars::schema_for;
use serde::Deserialize;

/// Application configuration.
#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
pub struct Config {
    /// The port to listen on.
    pub port: u16,
    /// Enables verbose logging.
    #[serde(default)]
    pub verbose: bool,
}

/// The JSON Schema of `Config`.
pub fn config_schema() -> serde_json::Value {
    serde_json::to_value(schema_for!(Config)).unwrap_or_default()
}

/// Validates `instance` against the schema.
pub fn is_valid(instance: &serde_json::Value) -> anyhow::Result<bool> {
    let validator = jsonschema::validator_for(&config_schema())?;
    Ok(validator.is_valid(instance))
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn round_trip() -> anyhow::Result<()> {
        assert!(is_valid(&json!({ "port": 8080 }))?);
        assert!(!is_valid(&json!({ "port": "eighty" }))?);
        assert!(!is_valid(&json!({ "verbose": true }))?);
        Ok(())
    }

    #[test]
    fn schema_lists_properties() {
        let schema = config_schema();
        assert!(schema["properties"]["port"].is_object());
    }
}
