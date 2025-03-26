//! Handle the `rbe` CLI subcommand

use clap::Arg;
use clap::ArgMatches;
use clap::Command;

use crate::CmdArgs;

/// Builds the `rbe` subcommand of the CLI user interface
pub(super) fn subcommand_rbe() -> Command {
    Command::new("rbe")
        //.visible_alias("")
        .about("Create the markdown for (a) Rust By Example book badge(s)")
        .display_order(3)
        .arg(
            Arg::new("concept")
                .required(true)
                .value_name("CONCEPT") // placeholder for the argument's value in the help message / usage.
                //.value_parser(value_parser!(...))
                .action(clap::ArgAction::Append)
                .help("Enter the RBE book chapter name e.g. \"attributes\""),
        )
}

/// Get the command line arguments for the `rbe` subcommand.
///
/// Returns `Some(CmdArgs)` if the `rbe` subcommand was used, `None` otherwise.
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs> {
    let mut badge = None;
    if let Some(m) = matches.subcommand_matches("rbe") {
        // "$ myapp rbe" was run
        let concepts = m
            .get_many::<String>("concept")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        badge = Some(CmdArgs { args: concepts });
    }
    badge
}
