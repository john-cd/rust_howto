//! Handle the `rbe` CLI subcommand

use clap::Arg;
use clap::ArgMatches;
use clap::Command;

/// The `rbe` subcommand arguments
#[derive(Debug)]
pub(crate) struct RbeCmdArgs {
    pub concepts: Vec<String>,
}

/// Builds the `rbe` subcommand of the CLI user interface
pub(super) fn subcommand_rbe() -> Command {
    Command::new("rbe")
        //.visible_alias("")
        .about("Create the markdown for a Rust By Example book badge")
        .display_order(2)
        .arg(
            Arg::new("concept")
                .required(true)
                .value_name("CONCEPT") // placeholder for the argument's value in the help message / usage.
                //.value_parser(value_parser!(...))
                .action(clap::ArgAction::Append)
                .help("Enter the RBE book chapter name e.g. \"attributes\""),
        )
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<RbeCmdArgs> {
    let mut badge = None;
    if let Some(m) = matches.subcommand_matches("rbe") {
        // "$ myapp rbe" was run
        let concepts = m
            .get_many::<String>("concept")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        badge = Some(RbeCmdArgs { concepts });
    }
    badge
}
