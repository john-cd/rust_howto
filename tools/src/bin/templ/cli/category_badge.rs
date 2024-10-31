//! Handle the `category_badge` CLI subcommand

use clap::ArgMatches;
use clap::Command;

/// Builds the `badge` subcommand of the CLI user interface
pub(super) fn subcommand_category_badge() -> Command {
    Command::new("category_badge")
        .visible_alias("c")
        .about("Create the markdown for all category badges for (a) given crate name(s)")
        //.display_order(1)
        .arg(super::arg_crate_name())
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<super::BadgeCmdArgs> {
    let mut badge = None;
    if let Some(m) = matches.subcommand_matches("category_badge") {
        // "$ myapp category_badge" was run
        let names = super::get_cmd_arg_crate_name(m);
        badge = Some(super::BadgeCmdArgs { names });
    }
    badge
}
