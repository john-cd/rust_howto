//! Handle the `badge` CLI subcommand

use clap::ArgMatches;
use clap::Command;

/// Builds the `badge` subcommand of the CLI user interface
pub(super) fn subcommand_badge() -> Command {
    Command::new("badge")
        .visible_alias("b")
        .about("Create the markdown for (a) crate badge(s)")
        .display_order(0)
        .arg(super::arg_crate_name())
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<crate::CratesCmdArgs> {
    let mut badge = None;
    if let Some(m) = matches.subcommand_matches("badge") {
        // "$ myapp badge" was run
        let names = super::get_cmd_arg_crate_name(m);
        badge = Some(crate::CratesCmdArgs { names });
    }
    badge
}
