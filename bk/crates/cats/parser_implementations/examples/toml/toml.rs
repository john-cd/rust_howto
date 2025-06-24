#![allow(dead_code)]
// ANCHOR: example
//! Example demonstrating TOML parsing and deserialization.

use serde::Deserialize;
use toml::value::Datetime;

#[derive(Deserialize, Debug)]
struct Config {
    title: String,
    database: Database,
}

#[derive(Deserialize, Debug)]
struct Database {
    server: String,
    ports: Vec<u32>,
    connection_max: u32,
    enabled: bool,
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

    let parsed_toml: Config = toml::from_str(toml_string).unwrap();
    println!("{:#?}", parsed_toml);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
