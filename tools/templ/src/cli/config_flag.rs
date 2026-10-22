//! Handle the global CLI argument `--verbose`
//!
//! TODO Reviewed ??-??-2026

use clap::Arg;
use clap::ArgMatches;

use crate::Config;

/// Defines the global `--verbose` flag.
pub(super) fn arg_verbose() -> Arg {
    Arg::new("verbose")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::SetTrue)
        .help("Use verbose output")
        .global(true)
}

/// Returns the configuration given by the global flags.
pub(super) fn get_config(matches: &ArgMatches) -> Config {
    Config {
        verbose: matches.get_flag("verbose"),
    }
}
