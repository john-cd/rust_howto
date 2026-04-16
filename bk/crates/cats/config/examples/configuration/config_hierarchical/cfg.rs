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
pub(crate) fn read_settings(cli_args: CommandLineArgs) -> Result<Settings> {
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

    if let Some(custom_config_file_path) = cli_args.custom_config_file_path {
        // Load settings from the custom file (which must exist), if a path
        // was provided. We could, for example, layer in a
        // local configuration file not checked in to `git`.
        cb = cb.add_source(File::from(custom_config_file_path).required(true));
    }

    cb = cb
        // Override settings with values from environment variables (with a prefix of
        // APP) e.g., `APP_DATABASE__MAX_CONNECTIONS=100`.
        .add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
                .ignore_empty(true),
        )
        // Programmatic override.
        // This will overwrite any value from other setting sources, if `port` is not `None`.
        .set_override_option("service.port", cli_args.port)?;

    // Reads all config sources and attempt to deserialize the entire
    // configuration into the `Settings` type.
    let config = cb.build()?;
    let s: Settings = config.try_deserialize()?;
    Ok(s)

    // // We could also get the value for a specific key:
    // let lvl = config.get("logging.level").context("Error getting
    // 'logging.level'")?;
    // // Or deserialize the config object into a HashMap:
    // println!(
    //         "\n{:?}",
    //         config.try_deserialize::<HashMap<String, String>>()?
    //  );
}
