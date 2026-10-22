// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
//! Example of using the `confy` crate for configuration management.
use serde::Deserialize;
use serde::Serialize;

/// Represents the configuration for the application.
#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    version: u8,
    api_key: String,
}

/// `MyConfig` implements `Default`:
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            version: 0,
            api_key: "".into(),
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: MyConfig = confy::load("my-app-name", None)?;
    // confy::store("my-app-name", None, cfg)?;
    println!("{cfg:?}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), confy::ConfyError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_defaults_from_an_isolated_config_path() -> anyhow::Result<()> {
        let dir = std::env::temp_dir().join(format!(
            "rust-howto-confy-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("my-app-name.toml");
        let cfg: MyConfig = confy::load_path(&path)?;
        assert_eq!(cfg.version, 0);
        assert!(cfg.api_key.is_empty());
        std::fs::remove_dir_all(dir)?;
        Ok(())
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/148)
