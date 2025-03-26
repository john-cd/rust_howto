//! Handle the `categories_badges_for_crate` CLI subcommand

use clap::ArgMatches;
use clap::Command;

use crate::CmdArgs;

/// Builds the `categories_badges_for_crate` subcommand of the CLI user
/// interface
pub(super) fn subcommand_category_badges_for_crate() -> Command {
    Command::new("categories_badges_for_crate")
        .visible_alias("cc")
        .about("Create the markdown for all category badges for (a) given crate(s)")
        .display_order(1)
        .arg(super::arg_crate_name())
}

/// Get the command-line arguments for the `categories_badges_for_crate` subcommand
///
/// # Arguments
/// * `matches` - The matches from the command line arguments
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("categories_badges_for_crate") {
        let names = super::get_cmd_arg_crate_name(m);
        cmdargs = Some(CmdArgs { args: names });
    }
    cmdargs
}
