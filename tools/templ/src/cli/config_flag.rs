//! Handle the global CLI argument `--verbose`
//!
//! TODO Reviewed ??-??-2026.

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

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use clap::Command;

    use super::*;

    /// Leaves verbose mode disabled when no flag is supplied.
    #[test]
    fn verbose_flag_defaults_to_false() {
        let matches = Command::new("templ")
            .arg(arg_verbose())
            .try_get_matches_from(["templ"])
            .expect("arguments should parse");

        let config = get_config(&matches);
        assert!(!config.verbose);
    }

    /// Enables verbose mode when the short flag is supplied.
    #[test]
    fn verbose_short_flag_enables_verbose() {
        let matches = Command::new("templ")
            .arg(arg_verbose())
            .try_get_matches_from(["templ", "-v"])
            .expect("arguments should parse");

        let config = get_config(&matches);
        assert!(config.verbose);
    }

    /// Enables verbose mode when the long flag is supplied.
    #[test]
    fn verbose_long_flag_enables_verbose() {
        let matches = Command::new("templ")
            .arg(arg_verbose())
            .try_get_matches_from(["templ", "--verbose"])
            .expect("arguments should parse");

        let config = get_config(&matches);
        assert!(config.verbose);
    }

    /// Applies the global verbose option to nested subcommands.
    #[test]
    fn verbose_flag_propagates_globally_with_subcommand() {
        let matches = Command::new("templ")
            .arg(arg_verbose())
            .subcommand(Command::new("sub"))
            .try_get_matches_from(["templ", "sub", "-v"])
            .expect("arguments should parse");

        let config = get_config(&matches);
        assert!(config.verbose);
    }
}
