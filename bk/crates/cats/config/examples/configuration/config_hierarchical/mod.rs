#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `config` crate to load
//! application configuration.
//!
//! It shows how to load and layer configuration from multiple sources,
//! including default settings, default config files, environment-specific
//! config files, and environment variables. It uses command-line arguments to
//! change what config files are loaded and to override specific configuration
//! options.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! config = "0.15.6" # Or latest
//! serde = { version = "1.0", features = ["derive"] }
//! clap = { version = "4.5.37", features = [ "cargo" ] } # Optional, for CLI argument parsing.
//! ```

use std::path::PathBuf;

use serde::Deserialize;

mod cfg;
mod cli;

pub(crate) use cfg::read_settings;
pub(crate) use cli::parse_command_line_args;

/// Application settings, which consist of nested configuration objects.
#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub service: ServiceSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ServiceSettings {
    /// The name of the application, here a service of some sort.
    pub name: String,
    /// The port the service listens on.
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DatabaseSettings {
    /// Connection URL.
    pub url: String,
    // Maximum number of connections.
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoggingSettings {
    pub level: String,
}

/// Command-line arguments for our application.
#[derive(Debug)]
pub(crate) struct CommandLineArgs {
    /// "production" or "development" environment.
    pub run_mode: String,
    /// Path to a custom config file.
    pub custom_config_file_path: Option<PathBuf>,
    /// CLI argument to override the port the service listens on.
    pub port: Option<u16>,
}

fn main() -> anyhow::Result<()> {
    // 1. Retrieve command-line arguments - in this example, default run_mode
    //    ("development"), no custom config file, and port value overriden.
    let cli_args = parse_command_line_args(Some(vec!["myapp", "-p8888"]));
    println!("Command-line arguments: {cli_args:?}\n");

    // 2. Read all setting sources:
    let settings = read_settings(cli_args)?;

    // 3. Use the settings:
    println!("Service Name: {}", settings.service.name); // From the environment variable.
    println!("Service Port: {}", settings.service.port); // Programmatic override.
    println!("Database URL: {}", settings.database.url); // From the base config file.
    println!(
        "Database Max Connections: {}",
        settings.database.max_connections
    ); // From `development.toml`.
    println!("Logging Level: {}", settings.logging.level); // From `development.toml`.

    Ok(())
}

/// Test function for the configuration example.
///
/// This function sets up temporary configuration files, runs the main
/// function, and cleans up afterwards.
fn test() -> anyhow::Result<()> {
    use std::fs;

    // Create the temp folder, if needed.
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }

    // Create the base configuration file.
    let default_toml = r#"
[service]
name = "UserService"
port = 8080

[database]
url = "postgresql://user:password@localhost:5432/mydb"
max_connections = 50

[logging]
level = "info"
"#;

    fs::write("temp/default.toml", default_toml)?;

    // Preparation: create a configuration file for the "development"
    // environment.
    let dev_toml = r#"
[database]
max_connections = 5

[logging]
level = "debug"
"#;

    fs::write("temp/development.toml", dev_toml)?;

    // Set environment variables to override specific settings, e.g.:
    // ```sh
    // export APP_SERVICE__PORT=9000
    // export APP_DATABASE__MAX_CONNECTIONS=25
    // export APP_LOGGING__LEVEL=error
    // ```
    unsafe {
        // Override the name in this example:
        std::env::set_var("APP_SERVICE__NAME", "MyService");
    }

    main()?;

    unsafe {
        std::env::remove_var("APP_SERVICE__NAME");
    }

    fs::remove_file("temp/default.toml")?;
    fs::remove_file("temp/development.toml")?;

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        use std::fs;

        // Create the temp folder, if needed.
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }

        // Create the base configuration file.
        let default_toml = r#"
    [service]
    name = "UserService"
    port = 8080

    [database]
    url = "postgresql://user:password@localhost:5432/mydb"
    max_connections = 50

    [logging]
    level = "info"
    "#;

        fs::write("temp/default.toml", default_toml)?;

        // Preparation: create a configuration file for the "development"
        // environment.
        let dev_toml = r#"
    [database]
    max_connections = 5

    [logging]
    level = "debug"
    "#;

        fs::write("temp/development.toml", dev_toml)?;

        // Set environment variables to override specific settings, e.g.:
        // ```sh
        // export APP_SERVICE__PORT=9000
        // export APP_DATABASE__MAX_CONNECTIONS=25
        // export APP_LOGGING__LEVEL=error
        // ```
        unsafe {
            // Override the name in this example:
            std::env::set_var("APP_SERVICE__NAME", "MyService");
        }

        main()?;

        unsafe {
            std::env::remove_var("APP_SERVICE__NAME");
        }

        fs::remove_file("temp/default.toml")?;
        fs::remove_file("temp/development.toml")?;

        Ok(())
    }
}
