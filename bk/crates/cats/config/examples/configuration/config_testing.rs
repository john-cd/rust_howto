#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Tests a config source without changing the process environment.

use std::collections::HashMap;

use config::Config;
use config::Environment;

/// Builds a configuration from a supplied set of environment values.
fn config_from_source(
    environment: HashMap<String, String>,
) -> Result<HashMap<String, String>, config::ConfigError> {
    let source = Environment::default().source(Some(environment));
    let config = Config::builder().add_source(source).build()?;
    config.try_deserialize()
}

/// Demonstrates an isolated environment source.
fn main() -> anyhow::Result<()> {
    let environment =
        HashMap::from([("HOST".to_owned(), "1.1.1.1".to_owned())]);
    let config = config_from_source(environment)?;
    assert_eq!(config.get("host"), Some(&"1.1.1.1".to_owned()));
    println!("Host: {}", config["host"]);
    Ok(())
}

// ANCHOR_END: example

/// Runs the isolated configuration-source example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies a provided environment map is read without process mutation.
    #[test]
    fn injected_environment_source_returns_the_expected_host()
    -> anyhow::Result<()> {
        let environment =
            HashMap::from([("HOST".to_owned(), "192.0.2.1".to_owned())]);
        let config = config_from_source(environment)?;
        assert_eq!(config.get("host"), Some(&"192.0.2.1".to_owned()));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
