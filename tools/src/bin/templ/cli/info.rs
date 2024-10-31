use clap::ArgMatches;
use clap::Command;

// The `info` command arguments
#[derive(Debug)]
pub(crate) struct InfoCmdArgs {
    pub names: Vec<String>,
}

/// Builds the `info` subcommand of the CLI user interface
pub(super) fn subcommand_info() -> Command {
    Command::new("info")
        .visible_alias("i")
        .about("Get crate information, including categories")
        .display_order(3)
        .arg(super::arg_crate_name())
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<InfoCmdArgs> {
    let mut info = None;
    if let Some(m) = matches.subcommand_matches("info") {
        // "$ myapp info" was run
        let names = super::get_cmd_arg_crate_name(m);
        info = Some(InfoCmdArgs { names });
    }
    info
}
