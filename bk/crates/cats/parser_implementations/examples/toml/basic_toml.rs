#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `basic-toml` crate to parse and
//! serialize TOML data.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! basic-toml = "0.1.9" # Or latest
//! chrono = { version = "0.4.40", features = [ "serde" ] }
//! serde = { version = "1.0.219", features = [ "derive" ] }
//! ```
use std::collections::HashMap;
use std::fs;

// Struct that will be serialized/deserialized.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    // Custom type.
    server: ServerConfig,
    // Hashmap.
    limits: HashMap<String, i64>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    // Option.
    debug: Option<bool>,
    // Vec.
    features: Vec<String>,
    // Custom serialization: timestamp in seconds since the epoch.
    #[serde(with = "chrono::serde::ts_seconds")]
    reset_datetime: chrono::DateTime<chrono::Utc>,
}

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
    let config: Config = basic_toml::from_str(toml_str)?;
    println!("Parsed config: {:#?}", config);

    // 2. Convert back to TOML string.
    let modified_toml = basic_toml::to_string(&config)?;
    println!("\nModified TOML:\n{}", modified_toml);

    // 3. Write TOML to file.
    fs::write("temp/config.toml", &modified_toml)?;

    // 4. Read TOML from file.
    let file_content = fs::read_to_string("temp/config.toml")?;
    let file_config: Config = basic_toml::from_str(&file_content)?;
    println!("\nConfig from file: {:#?}", file_config);

    // 5. Error handling.
    let invalid_toml = r#"
        [server]
        host = 127.0.0.1  # Missing quotes around string
        port = "8080"     # String instead of integer
        # Missing fields
    "#;

    match basic_toml::from_str::<Config>(invalid_toml) {
        Ok(config) => println!("Successfully parsed: {:?}", config),
        Err(e) => println!("Parse error: {}", e),
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
