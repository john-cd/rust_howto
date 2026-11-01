// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates parsing INI text and using typed getters with
//! `configparser`.
//!
//! INI is a simple configuration format of `[sections]` containing `key =
//! value` lines, used on Windows and by many tools. It has no standard for
//! types, so the typed getters convert text to numbers and booleans.

use anyhow::Context;
use anyhow::anyhow;
use configparser::ini::Ini;

/// Parses an INI configuration string and prints its host, port, and serialized
/// form.
fn main() -> anyhow::Result<()> {
    let input = "[server]\nhost = 127.0.0.1\nport = 8080\n";
    let mut config = Ini::new();
    config
        .read(input.to_owned())
        .map_err(anyhow::Error::msg)
        .context("parse server configuration")?;

    let host = config
        .get("SERVER", "HOST")
        .ok_or_else(|| anyhow!("missing [server] host"))?;
    let port = config
        .getuint("server", "port")
        .map_err(anyhow::Error::msg)?
        .ok_or_else(|| anyhow!("missing [server] port"))?;

    println!("Listening on {host}:{port}");

    let _previous_value = config.setstr("server", "host", Some("0.0.0.0"));
    println!("Updated configuration:\n{}", config.writes());
    Ok(())
}

// ANCHOR_END: example

/// Runs the `configparser` example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the INI parsing example completes successfully.
    #[test]
    fn test_configparser_ini_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Keys are case-insensitive and typed getters parse the value.
    #[test]
    fn reads_typed_values() -> anyhow::Result<()> {
        let mut config = Ini::new();
        config
            .read("[server]\nhost = 127.0.0.1\nport = 8080\n".to_owned())
            .map_err(anyhow::Error::msg)?;
        assert_eq!(config.get("SERVER", "HOST").as_deref(), Some("127.0.0.1"));
        assert_eq!(
            config
                .getuint("server", "port")
                .map_err(anyhow::Error::msg)?,
            Some(8080)
        );
        assert_eq!(config.get("server", "missing"), None);
        Ok(())
    }

    /// A non-numeric port is an error, not a silent default.
    #[test]
    fn invalid_number_is_an_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Ini::new();
        config.read("[server]\nport = abc\n".to_owned())?;
        assert!(config.getuint("server", "port").is_err());
        Ok(())
    }

    /// Updated values show up in the serialized configuration.
    #[test]
    fn updated_value_is_written() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Ini::new();
        config.read("[server]\nhost = 127.0.0.1\n".to_owned())?;
        config.setstr("server", "host", Some("0.0.0.0"));
        assert!(config.writes().contains("host=0.0.0.0"));
        Ok(())
    }
}
