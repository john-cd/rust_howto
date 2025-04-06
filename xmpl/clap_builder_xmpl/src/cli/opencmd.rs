use std::path::PathBuf;

use clap::ArgMatches;
use clap::Command;

use super::args::arg_filepaths;
use super::args::get_arg_filepaths;

/// Defines the `<exec> open file`.
pub(super) fn cmd() -> Command {
    Command::new("open")
        // We could also use: .short_flag('o') or .visible_alias("o")
        .about("Open one or more files")
        .arg(arg_filepaths().required(true)) // the main program's
    // argument is reused
    // here, but is
    // marked required.
}

pub(super) fn get_args(matches: &ArgMatches) -> Option<Vec<PathBuf>> {
    let m = matches.subcommand_matches("open")?;
    // "$ myapp open ..." was run
    get_arg_filepaths(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::cli;

    #[test]
    fn verify_open_cmd() {
        cmd().debug_assert(); // https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert
    }

    #[test]
    fn test_cmd_open() {
        let m = cli().get_matches_from(vec!["foo", "open", "a.csv", "b.csv"]);
        assert_eq!(
            get_args(&m),
            Some(vec![PathBuf::from("a.csv"), PathBuf::from("b.csv")])
        );
    }

    #[test]
    fn test_cmd_open_none() {
        let m = cli().try_get_matches_from(vec!["foo", "open"]);
        assert!(m.is_err());
    }
}
