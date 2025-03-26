use clap::ArgMatches;
use clap::Command;

/// Builds the `info` subcommand of the CLI user interface.
///
/// This subcommand is used to retrieve information about a crate,
/// including its categories.
pub(super) fn subcommand_info() -> Command {
    Command::new("info")
        .visible_alias("i")
        .about("Get crate information, including categories")
        .display_order(4)
        // Add the crate name argument to the subcommand.
        .arg(super::arg_crate_name())
}

/// "$ myapp info" was run
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<crate::CmdArgs> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("info") {
        let names = super::get_cmd_arg_crate_name(m);
        cmdargs = Some(crate::CmdArgs { args: names });
    }
    cmdargs
}
