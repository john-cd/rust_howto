#![cfg(target_family = "unix")]
// ANCHOR: example
//! This example demonstrates how to read an environment variable and use it to
//! determine the path to a configuration file. If the environment variable is
//! not set, a default path is used.

use std::env;
use std::fs;
use std::io::Error;

/// Reads a configuration file from a path specified by the `CONFIG` environment
/// variable, or a default path if the variable is not set.
fn main() -> Result<(), Error> {
    let config_path =
        env::var("CONFIG").unwrap_or("/etc/subversion/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
