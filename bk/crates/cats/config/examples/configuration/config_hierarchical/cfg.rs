// TODO Reviewed ??-??-2026.

//! Builds typed settings from files, environment variables, and CLI overrides.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use config::Config;
use config::Environment;
use config::File;

use super::CommandLineArgs;
use super::Settings;

/// Reads settings in increasing precedence: defaults, files, environment, then
/// CLI.
pub(crate) fn read_settings(
    cli_args: CommandLineArgs,
    config_dir: &Path,
    environment_source: Option<HashMap<String, String>>,
) -> Result<Settings> {
    let mut builder = Config::builder()
        .set_default("service.port", 80)?
        .add_source(File::from(config_dir.join("default.toml")))
        .add_source(
            File::from(config_dir.join(format!("{}.toml", cli_args.run_mode)))
                .required(false),
        );

    if let Some(custom_config_file_path) = cli_args.custom_config_file_path {
        builder = builder
            .add_source(File::from(custom_config_file_path).required(true));
    }

    let mut environment = Environment::with_prefix("APP")
        .prefix_separator("_")
        .separator("__")
        .ignore_empty(true);
    if let Some(source) = environment_source {
        environment = environment.source(Some(source));
    }

    let config = builder
        .add_source(environment)
        .set_override_option("service.port", cli_args.port)?
        .build()?;
    Ok(config.try_deserialize()?)
}
