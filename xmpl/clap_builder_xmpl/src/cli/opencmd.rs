//! The `open` subcommand.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use clap::ArgMatches;
use clap::Command;

use super::args::arg_filepaths;
use super::args::get_arg_filepaths;

/// Defines the `open` command, which accepts one or more file paths.
pub(super) fn cmd() -> Command {
    Command::new("open")
        // An optional short flag or visible alias could provide the `-o` shortcut.
        .about("Open one or more files")
        .arg(arg_filepaths().required(true))
}

/// Returns the files given to the `open` command, if it was run.
pub(super) fn get_args(matches: &ArgMatches) -> Option<Vec<PathBuf>> {
    let m = matches.subcommand_matches("open")?;
    // The command line included `myapp open ...`.
    get_arg_filepaths(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::cli;

    /// Lets clap check the command definition for mistakes.
    #[test]
    fn verify_open_cmd() {
        cmd().debug_assert(); // <https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert>
    }

    /// Checks that the files are collected.
    #[test]
    fn test_cmd_open() {
        let m = cli().get_matches_from(vec!["foo", "open", "a.csv", "b.csv"]);
        assert_eq!(
            get_args(&m),
            Some(vec![PathBuf::from("a.csv"), PathBuf::from("b.csv")])
        );
    }

    /// Checks that at least one file is required.
    #[test]
    fn test_cmd_open_none() {
        let m = cli().try_get_matches_from(vec!["foo", "open"]);
        assert!(m.is_err());
    }
}
