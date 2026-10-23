#![allow(dead_code)]
// TODO Reviewed ??-??-2026

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
/// It uses `toml::from_str` to parse a string into a
/// `Config` struct.
fn main() {
    let toml_string = r#"
        title = "TOML Example"

        [database]
        server = "192.168.1.1"
        ports = [ 8001, 8001, 8002 ]
        connection_max = 5000
        enabled = true
        date = 1979-05-27T07:32:00Z
    "#;

    let parsed_toml: Config =
        toml::from_str(toml_string).expect("Failed to parse TOML");
    println!("{parsed_toml:#?}");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
