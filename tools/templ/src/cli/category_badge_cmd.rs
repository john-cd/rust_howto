//! Handle the `category_badge` CLI subcommand

use clap::ArgMatches;
use clap::Command;

use crate::CmdArgs;

/// Builds the `category_badge` subcommand of the CLI user interface
pub(super) fn subcommand_category_badge() -> Command {
    Command::new("category_badge")
        .visible_alias("c")
        .about("Create the markdown for category badge(s), given the category(ies)")
        .display_order(2)
        .arg(
            clap::Arg::new("category")
                .required(true)
                .value_name("CATEGORY") // placeholder for the argument's value in the help message / usage.
                .action(clap::ArgAction::Append)
                .help("Enter the category name(s)"),
        )
}

/// Returns the command line arguments for the `category_badge` subcommand
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("category_badge") {
        let categories = m
            .get_many::<String>("category")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        cmdargs = Some(CmdArgs { args: categories });
    }
    cmdargs
}
