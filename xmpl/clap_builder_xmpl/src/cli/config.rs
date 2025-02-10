//! Configuration struct and the associated builder.

use std::path::PathBuf;

pub(crate) struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub(crate) fn set_verbose(mut self, verbose: u8) -> Self {
        self.config.verbose = verbose;
        self
    }

    pub(crate) fn set_config_file(mut self, config_file: Option<PathBuf>) -> Self {
        self.config.config_file = config_file;
        self
    }

    pub(crate) fn build(self) -> Config {
        self.config
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Config {
    verbose: u8,
    config_file: Option<PathBuf>,
}

impl Config {
    pub(crate) fn builder() -> ConfigBuilder {
        ConfigBuilder {
            config: Self::default(),
        }
    }

    pub(crate) fn get_verbose(&self) -> u8 {
        self.verbose
    }

    pub(crate) fn get_config_file(&self) -> Option<PathBuf> {
        self.config_file.clone()
    }
}
