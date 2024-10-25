//! Handle the `badge` CLI subcommand

use clap::Arg;
use clap::ArgMatches;
use clap::Command;

// The `badge` command arguments
#[derive(Debug)]
pub(crate) struct BadgeCmdArgs {
    pub names: Vec<String>,
}

/// Builds the `badge` subcommand of the CLI user interface
pub(super) fn subcommand_badge() -> Command {
    Command::new("badge")
        .visible_alias("b")
        .about("Create the markdown for a crate badge")
        .display_order(0)
        .arg(
            Arg::new("crate_name")
                .required(true)
                .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
                .action(clap::ArgAction::Append)
                .help("Enter the crate name(s)"),
        )
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<BadgeCmdArgs> {
    let mut badge = None;
    if let Some(m) = matches.subcommand_matches("badge") {
        // "$ myapp badge" was run
        let names = m
            .get_many::<String>("crate_name")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        badge = Some(BadgeCmdArgs { names });
    }
    badge
}
