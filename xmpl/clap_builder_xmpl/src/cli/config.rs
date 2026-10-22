//! Configuration struct and the associated builder.
//!
//! TODO Reviewed ??-??-2026

use std::path::PathBuf;

/// Builds a [`Config`] step by step.
pub(crate) struct ConfigBuilder {
    /// The configuration being built.
    config: Config,
}

impl ConfigBuilder {
    /// Sets the verbosity level.
    pub(crate) fn set_verbose(mut self, verbose: u8) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Sets the path of the configuration file.
    pub(crate) fn set_config_file(mut self, config_file: Option<PathBuf>) -> Self {
        self.config.config_file = config_file;
        self
    }

    /// Returns the configuration.
    pub(crate) fn build(self) -> Config {
        self.config
    }
}

/// The configuration of the application.
#[derive(Default, Clone, Debug)]
pub(crate) struct Config {
    /// Verbosity level; the number of `-v` flags.
    verbose: u8,
    /// Path of the configuration file, if any.
    config_file: Option<PathBuf>,
}

impl Config {
    /// Returns a builder for a configuration.
    pub(crate) fn builder() -> ConfigBuilder {
        ConfigBuilder {
            config: Self::default(),
        }
    }

    /// Returns the verbosity level.
    pub(crate) fn get_verbose(&self) -> u8 {
        self.verbose
    }

    /// Returns the path of the configuration file, if any.
    pub(crate) fn get_config_file(&self) -> Option<PathBuf> {
        self.config_file.clone()
    }
}
