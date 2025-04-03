//! Note: The `config` package has the same name as the current crate.
//! To avoid naming conflicts, we've renamed it to `config1` in `Cargo.toml`.
use config1 as config;
// ANCHOR: example
// This example demonstrates how to use the `config` crate to manage
// application configuration, particularly from environment variables.

/// Retrieves the application's configuration.
fn config() -> &'static config::Config {
    // Get or initialize the configuration.
    // (`OnceLock` can be written to only once).
    static CONFIG: std::sync::OnceLock<config::Config> =
        std::sync::OnceLock::new();
    CONFIG.get_or_init(|| {
        config::Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap()
    })
}

/// Retrieves a configuration value by key.
///
/// This function fetches a value from the static configuration object
/// based on the provided key. It deserializes the value into the
/// specified type `T`.
fn get<'a, T: serde::Deserialize<'a>>(
    key: &str,
) -> Result<T, config::ConfigError> {
    config().get::<T>(key)
}

fn main() -> anyhow::Result<()> {
    // Set the environment variable for testing.
    unsafe {
        std::env::set_var("APP_PORT", "8080");
    }

    // Retrieve and print the "port" configuration value.
    println!("{:?}", get::<String>("port")?);

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
