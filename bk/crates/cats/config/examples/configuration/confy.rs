#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Loads a typed configuration with confy.

use serde::Deserialize;
use serde::Serialize;

/// Application settings stored by confy.
#[derive(Serialize, Deserialize, Debug, Default)]
struct MyConfig {
    /// The configuration schema version.
    version: u8,
    /// The API key.
    api_key: String,
}

/// Loads defaults from an isolated TOML path and displays them.
fn main() -> anyhow::Result<()> {
    let config_dir = tempfile::tempdir()?;
    let path = config_dir.path().join("my-app.toml");
    let config: MyConfig = confy::load_path(path)?;
    println!("{config:?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the confy example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies confy supplies defaults for a new temporary config file.
    #[test]
    fn loads_defaults_from_an_isolated_config_path() -> anyhow::Result<()> {
        let config_dir = tempfile::tempdir()?;
        let path = config_dir.path().join("my-app.toml");
        let config: MyConfig = confy::load_path(path)?;
        assert_eq!(config.version, 0);
        assert!(config.api_key.is_empty());
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
