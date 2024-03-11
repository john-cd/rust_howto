use std::path::PathBuf;

use clap::ArgMatches;
use clap::Command;

use super::arg_open;
use super::get_arg_open;

/// Defines the `<exec> open file`
pub(super) fn cmd() -> Command {
    Command::new("open")
            // We could also use: .short_flag('o') or .visible_alias("o")
            .about("Open one or more files")
            .arg(arg_open().required(true))
}

pub(super) fn get_args(matches: &ArgMatches) -> Vec<PathBuf> {
    if let Some(m) = matches.subcommand_matches("open") {
        // "$ myapp open ..." was run
        get_arg_open(m)
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_open_cmd() {
        cmd().debug_assert(); // https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert
    }
}
