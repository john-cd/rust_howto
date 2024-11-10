//! Handle the global CLI argument `--verbose`

use clap::Arg;
use clap::ArgMatches;

use crate::Config;

pub(super) fn arg_verbose() -> Arg {
    Arg::new("verbose")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::SetTrue)
        .help("Use verbose output")
        .global(true)
}

pub(super) fn get_config(matches: &ArgMatches) -> Config {
    Config {
        verbose: matches.get_flag("verbose"),
    }
}
