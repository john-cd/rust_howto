#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the maintained `toml` crate to parse
//! and serialize TOML data.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! toml = "1.1"
//! chrono = { version = "0.4.40", features = [ "serde" ] }
//! serde = { version = "1.0.219", features = [ "derive" ] }
//! ```

use std::collections::HashMap;
use std::fs;

// Struct that will be serialized/deserialized.
/// The configuration, which is serialized to and deserialized from TOML.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    // Custom type.
    /// The server settings, as a nested table.
    server: ServerConfig,
    // Hashmap.
    /// The limits, as a map from names to values.
    limits: HashMap<String, i64>,
}

/// The settings of the server.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ServerConfig {
    /// The host name or address.
    host: String,
    /// The port to listen on.
    port: u16,
    // Option.
    /// Whether debugging is enabled, if specified.
    debug: Option<bool>,
    // Vec.
    /// The enabled features.
    features: Vec<String>,
    // Custom serialization: timestamp in seconds since the epoch.
    /// The date and time of the reset, stored as seconds since the epoch.
    #[serde(with = "chrono::serde::ts_seconds")]
    reset_datetime: chrono::DateTime<chrono::Utc>,
}

/// Parses TOML, modifies it, writes it to a file and reads it back, then
/// handles an invalid document.
fn main() -> anyhow::Result<()> {
    // 1. Parse TOML from a string.
    let toml_str = r#"

        [server]
        host = "127.0.0.1"
        port = 8080
        reset_datetime = 1431684000
        features = ["auth", "api", "webhooks"]

        [limits]
        requests_per_minute = 60
        max_payload_size = 1048576
    "#;

    // Parse the TOML string into our `Config` struct.
    let config: Config = toml::from_str(toml_str)?;
    println!("Parsed config: {config:#?}");

    // 2. Convert back to TOML string.
    let modified_toml = toml::to_string(&config)?;
    println!("\nModified TOML:\n{modified_toml}");

    // 3. Write TOML to file.
    fs::write("temp/config.toml", &modified_toml)?;

    // 4. Read TOML from file.
    let file_content = fs::read_to_string("temp/config.toml")?;
    let file_config: Config = toml::from_str(&file_content)?;
    println!("\nConfig from file: {file_config:#?}");

    // 5. Error handling.
    let invalid_toml = r#"
        [server]
        host = 127.0.0.1  # Missing quotes around string
        port = "8080"     # String instead of integer
        # Missing fields
    "#;

    match toml::from_str::<Config>(invalid_toml) {
        Ok(config) => println!("Successfully parsed: {config:?}"),
        Err(e) => println!("Parse error: {e}"),
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the basic toml example's main path succeeds.
    /// Runs the basic toml example through its entry point.
    #[test]
    fn test_basic_toml_example_entry_point() -> anyhow::Result<()> {
        use std::fs;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        main()?;
        Ok(())
    }
}
