#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to load application configuration and store it
//! in a singleton.
//!
//! Add to `Cargo.toml`:
//! ```toml
//! config = "0.15.6" # Or latest
//! ```

mod settings {
    use std::sync::OnceLock;

    use ::config::Config;
    use ::config::ConfigError;
    use serde::Deserialize;

    // Centralized access point to the configuration.
    fn config() -> &'static Config {
        // The configuration is stored in a `static`.
        // `OnceLock` can be written to only once and is thread-safe.
        static CONFIG: OnceLock<Config> = OnceLock::new();

        // Get the config, initializing it if it was uninitialized.
        CONFIG.get_or_init(|| {
            Config::builder()
                .add_source(config::Environment::with_prefix("APP"))
                .build() // Read the environment variables starting with `APP`.
                .unwrap()
        })
    }

    /// Retrieves a configuration value by key.
    ///
    /// This function fetches a value from the static configuration object
    /// based on the provided key. It deserializes the value into the
    /// specified type `T`.
    pub fn get<'a, T: Deserialize<'a>>(key: &str) -> Result<T, ConfigError> {
        config().get::<T>(key)
        // In our case, we could also use `get_string`.
    }
}

fn main() -> anyhow::Result<()> {
    // Set an environment variable for testing.
    unsafe {
        std::env::set_var("APP_PORT", "8080");
    }

    // Retrieve and print the "port" configuration value.
    println!("{:?}", settings::get::<String>("port")?);

    // Remove the environment variable after testing.
    unsafe {
        std::env::remove_var("APP_Port");
    }
    Ok(())
}
// Adapted from: https://github.com/rust-cli/config-rs/blob/main/examples/static_env.rs
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
