use clap::ArgMatches;
use clap::Command;

/// Builds the `info` subcommand of the CLI user interface
pub(super) fn subcommand_info() -> Command {
    Command::new("info")
        .visible_alias("i")
        .about("Get crate information, including categories")
        .display_order(4)
        .arg(super::arg_crate_name())
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<crate::CmdArgs> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("info") {
        // "$ myapp info" was run
        let names = super::get_cmd_arg_crate_name(m);
        cmdargs = Some(crate::CmdArgs { args: names });
    }
    cmdargs
}
