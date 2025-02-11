// HACK: the `config` package has the same name
// than the current crate, thus we renamed it `config1`
// See `Cargo.toml`
use config1 as config;

// ANCHOR: example
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

/// Get a configuration value from the static configuration object
fn get<'a, T: serde::Deserialize<'a>>(
    key: &str,
) -> Result<T, config::ConfigError> {
    config().get::<T>(key)
}

fn main() -> anyhow::Result<()> {
    // Prep: set the environment variable
    unsafe {
        std::env::set_var("APP_PORT", "8080");
    }
    println!("{:?}", get::<String>("port")?);
    unsafe {
        std::env::remove_var("APP_Port");
    }
    Ok(())
}
// Adapted from https://github.com/rust-cli/config-rs/blob/main/examples/static_env.rs
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
