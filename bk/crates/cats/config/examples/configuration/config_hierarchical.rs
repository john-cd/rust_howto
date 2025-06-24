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

/// Retrieve the settings for our application.
mod cfg {

    use anyhow::Result;
    use config::Config;
    use config::ConfigBuilder;
    use config::Environment;
    use config::File;

    use super::CommandLineArgs;
    use super::Settings;

    static DEFAULT_CONFIG_FILE: &str = "temp/default";

    /// Read the application's settings from multiple sources.
    ///
    /// This function layers configuration from a default config file, an
    /// environment-specific file (if it exists), a custom config file (if
    /// provided), environment variables, and programmatic overrides.
    pub(super) fn read_settings(cli_args: CommandLineArgs) -> Result<Settings> {
        let mut cb: ConfigBuilder<_> = Config::builder()
            // Provide defaults for some settings.
            // Default settings will be overwritten by any source or override providing a value for the same key.
            .set_default("service.port", 80)?
            // Load settings from a base config file.
            // `File::with_name(..)` is shorthand for `File::from(Path::new(..))`.
            // The extension is implicit and could be TOML, JSON, YAML, INI, etc.
            .add_source(File::with_name(DEFAULT_CONFIG_FILE))
            // Load settings from an run-mode-specific config file, if it is present.
            .add_source(File::with_name(&format!("temp/{}", cli_args.run_mode)).required(false));

        if let Some(custom_config_file_path) = cli_args.custom_config_file_path
        {
            // Load settings from the custom file (which must exist), if a path
            // was provided. You could, for example, layer in a
            // local configuration file not checked in to `git`.
            cb = cb
                .add_source(File::from(custom_config_file_path).required(true));
        }

        cb = cb
            // Override settings with values from environment variables (with a prefix of
            // APP) e.g., `APP_DATABASE__MAX_CONNECTIONS=100`
            .add_source(Environment::with_prefix("APP").prefix_separator("_").separator("__").ignore_empty(true))
            // Programmatic override.
            // This will overwrite any value from other setting sources, if `port` is not `None`.
            .set_override_option("service.port", cli_args.port)?;

        // Reads all config sources and attempt to deserialize the entire
        // configuration into the `Settings` type.
        let config = cb.build()?;
        let s: Settings = config.try_deserialize()?;
        Ok(s)

        // You could also get the value for a specific key:
        // let lvl = config.get("logging.level").context("Error getting
        // 'logging.level'")?; Or deserialize the config object into a
        // HashMap:
        // println!(
        //         "\n{:?}",
        //         config.try_deserialize::<HashMap<String, String>>()?
        //  );
    }
}

/// Command-line arguments for our application.
mod cli {

    use std::ffi::OsString;
    use std::path::PathBuf;

    use clap::Arg;
    use clap::arg;
    use clap::command;
    use clap::value_parser;

    use super::CommandLineArgs;

    /// Define the command-line arguments.
    fn cmd() -> clap::Command {
        command!()
        // Optional run mode (a.k.a. "environment"), which can be "production" or "development" and defaults to the latter.
        .arg(Arg::new("run_mode").short('e').long("env").required(false).value_parser(["production", "development"]).default_value("development"))
        // Optional custom config file.
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        // Optional port number.
        .arg(
            arg!(-p --port <PORT> "Sets the service's port")
                .required(false)
                .value_parser(value_parser!(u16))
        )
    }

    /// Parses command-line arguments.
    ///
    /// This function parses command-line arguments using the `clap` crate.
    /// - Run mode (with default value),
    /// - Optional custom config file path,
    /// - Optional port number override.
    ///
    /// It can also be used for testing by passing in a custom set of arguments.
    pub fn parse_command_line_args<I, T>(opt_args: Option<I>) -> CommandLineArgs
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // Get command-line arguments from `std::env::args_os`,
        // unless they are passed manually in `opt_args`
        // (e.g. for testing purposes).
        let matches = if let Some(args) = opt_args {
            cmd().get_matches_from(args)
        } else {
            cmd().get_matches()
        };

        let run_mode: String = matches
            .get_one::<String>("run_mode")
            .expect("run_mode has a default value.")
            .clone();

        // Get the provided path to the custom config file, if any.
        let custom_config_file_path =
            matches.get_one::<PathBuf>("config").cloned();

        let port: Option<u16> = matches.get_one("port").copied();

        CommandLineArgs {
            run_mode,
            custom_config_file_path,
            port,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn verify_cmd() {
            cmd().debug_assert();
        }
    }
}

fn main() -> anyhow::Result<()> {
    // 1. Retrieve command-line arguments - in this example, default run_mode
    //    ("development"), no custom config file, and port value overriden.
    let cli_args = cli::parse_command_line_args(Some(vec!["myapp", "-p8888"]));
    println!("Command-line arguments: {:?}\n", cli_args);

    // 2. Read all setting sources:
    let settings = cfg::read_settings(cli_args)?;

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
// ANCHOR_END: example
