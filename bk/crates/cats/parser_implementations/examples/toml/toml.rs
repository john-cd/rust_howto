// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Example demonstrating TOML parsing and deserialization.

use serde::Deserialize;
use toml::value::Datetime;

/// The configuration.
#[derive(Deserialize, Debug)]
struct Config {
    /// The title of the configuration.
    title: String,
    /// The database settings.
    database: Database,
}

/// The settings of the database.
#[derive(Deserialize, Debug)]
struct Database {
    /// The address of the database server.
    server: String,
    /// The ports of the database server.
    ports: Vec<u32>,
    /// The maximum number of connections.
    connection_max: u32,
    /// Whether the database is enabled.
    enabled: bool,
    /// The date and time of the last update.
    date: Datetime,
}

/// Demonstrates parsing a TOML string into a `Config` struct.
///
/// # Errors
///
/// Returns an error if the example TOML cannot be deserialized.
fn main() -> anyhow::Result<()> {
    let toml_string = r#"
        title = "TOML Example"

        [database]
        server = "192.168.1.1"
        ports = [ 8001, 8001, 8002 ]
        connection_max = 5000
        enabled = true
        date = 1979-05-27T07:32:00Z
    "#;

    let parsed_toml: Config = toml::from_str(toml_string)?;
    println!("{parsed_toml:#?}");

    Ok(())
}

// ANCHOR_END: example

/// Runs the example and propagates deserialization errors.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the toml example through its entry point.
    #[test]
    fn test_toml_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// The sample document is parsed into the typed structure.
    #[test]
    fn parses_the_sample() {
        let config: Config = toml::from_str(
            "title = \"T\"\n[database]\nserver = \"s\"\nports = [1, 2]\n\
             connection_max = 5\nenabled = true\ndate = 1979-05-27T07:32:00Z\n",
        )
        .expect("valid TOML");
        assert_eq!(config.title, "T");
        assert_eq!(config.database.ports, [1, 2]);
        assert!(config.database.enabled);
        assert_eq!(config.database.date.to_string(), "1979-05-27T07:32:00Z");
    }

    /// A missing table is an error.
    #[test]
    fn missing_table_is_an_error() {
        assert!(toml::from_str::<Config>("title = \"T\"").is_err());
    }
}
