#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Loads application settings once with a thread-safe singleton.
//!
//! Add config and serde with the derive feature to Cargo.toml.

/// Application settings, loaded once and shared.
mod settings {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    use anyhow::anyhow;
    use config::Config;
    use config::ConfigError;
    use config::Environment;
    use serde::de::DeserializeOwned;

    /// Builds settings from the process environment or an injected test source.
    fn build_config(
        environment_source: Option<HashMap<String, String>>,
    ) -> Result<Config, ConfigError> {
        let mut environment = Environment::with_prefix("APP");
        if let Some(source) = environment_source {
            environment = environment.source(Some(source));
        }
        Config::builder()
            .set_default("port", 8080)?
            .add_source(environment)
            .build()
    }

    /// Returns the process configuration, initializing it on first access.
    fn config() -> anyhow::Result<&'static Config> {
        static CONFIG: OnceLock<Result<Config, String>> = OnceLock::new();

        match CONFIG.get_or_init(|| {
            build_config(None).map_err(|error| error.to_string())
        }) {
            Ok(config) => Ok(config),
            Err(error) => Err(anyhow!(error.clone())),
        }
    }

    /// Retrieves a typed setting from the cached process configuration.
    pub fn get<T: DeserializeOwned>(key: &str) -> anyhow::Result<T> {
        Ok(config()?.get::<T>(key)?)
    }

    /// Builds an isolated configuration from supplied environment values.
    #[cfg(test)]
    pub(super) fn from_source(
        environment_source: HashMap<String, String>,
    ) -> Result<Config, ConfigError> {
        build_config(Some(environment_source))
    }
}

/// Reads the port and displays it.
fn main() -> anyhow::Result<()> {
    let port = settings::get::<u16>("port")?;
    println!("Service port: {port}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the singleton configuration example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that an injected environment value overrides the default port.
    #[test]
    fn environment_source_overrides_the_default_without_process_mutation()
    -> anyhow::Result<()> {
        let source = std::collections::HashMap::from([(
            "APP_PORT".to_owned(),
            "9000".to_owned(),
        )]);
        let config = settings::from_source(source)?;
        assert_eq!(config.get::<u16>("port")?, 9000);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
