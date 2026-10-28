#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Loads typed settings by layering TOML files, environment variables, and CLI
//! overrides.
//!
//! The example uses temporary TOML files so it can run without project setup.
//! In an application, point the same File sources at the checked-in config
//! paths.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

mod cfg;
mod cli;

pub(crate) use cfg::read_settings;
pub(crate) use cli::parse_command_line_args;

/// Application settings loaded from several sources.
#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    /// Settings for the service.
    pub service: ServiceSettings,
    /// Settings for the database.
    pub database: DatabaseSettings,
    /// Settings for logging.
    pub logging: LoggingSettings,
}

/// Service settings.
#[derive(Debug, Deserialize)]
pub(crate) struct ServiceSettings {
    /// The application name.
    pub name: String,
    /// The port the service listens on.
    pub port: u16,
}

/// Database settings.
#[derive(Debug, Deserialize)]
pub(crate) struct DatabaseSettings {
    /// The connection URL.
    pub url: String,
    /// The maximum connection count.
    pub max_connections: u32,
}

/// Logging settings.
#[derive(Debug, Deserialize)]
pub(crate) struct LoggingSettings {
    /// The logging level.
    pub level: String,
}

/// Command-line options that override configuration sources.
#[derive(Debug)]
pub(crate) struct CommandLineArgs {
    /// The selected configuration profile.
    pub run_mode: String,
    /// An optional additional configuration file.
    pub custom_config_file_path: Option<PathBuf>,
    /// An optional service port override.
    pub port: Option<u16>,
}

/// Writes example TOML files into a caller-owned temporary directory.
fn write_sample_files(config_dir: &Path) -> Result<()> {
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
    std::fs::write(config_dir.join("default.toml"), default_toml)?;

    let development_toml = r#"
[database]
max_connections = 5

[logging]
level = "debug"
"#;
    std::fs::write(config_dir.join("development.toml"), development_toml)?;
    Ok(())
}

/// Loads and displays settings after applying all configured overrides.
fn main() -> Result<()> {
    let config_dir = tempfile::tempdir()?;
    write_sample_files(config_dir.path())?;

    let cli_args =
        parse_command_line_args(Some(vec!["myapp", "--port", "8888"]));
    let settings = read_settings(cli_args, config_dir.path(), None)?;

    assert_eq!(settings.service.port, 8888);
    println!("Service name: {}", settings.service.name);
    println!("Service port: {}", settings.service.port);
    println!("Database URL: {}", settings.database.url);
    println!(
        "Database connections: {}",
        settings.database.max_connections
    );
    println!("Logging level: {}", settings.logging.level);
    Ok(())
}

// ANCHOR_END: example

/// Runs the layered configuration example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// Verifies that defaults, files, environment values, and CLI overrides
    /// layer in order.
    #[test]
    fn cli_overrides_environment_and_toml_settings() -> anyhow::Result<()> {
        let config_dir = tempfile::tempdir()?;
        write_sample_files(config_dir.path())?;

        let cli_args =
            parse_command_line_args(Some(vec!["myapp", "--port", "8888"]));
        let environment = HashMap::from([
            ("APP_SERVICE__NAME".to_owned(), "MyService".to_owned()),
            ("APP_DATABASE__MAX_CONNECTIONS".to_owned(), "25".to_owned()),
        ]);
        let settings =
            read_settings(cli_args, config_dir.path(), Some(environment))?;

        assert_eq!(settings.service.name, "MyService");
        assert_eq!(settings.service.port, 8888);
        assert_eq!(
            settings.database.url,
            "postgresql://user:password@localhost:5432/mydb"
        );
        assert_eq!(settings.database.max_connections, 25);
        assert_eq!(settings.logging.level, "debug");
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
