use clap::Arg;
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
        .display_order(2)
        .arg(
            Arg::new("crate_name")
                .required(true)
                .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
                .action(clap::ArgAction::Append)
                .help("Enter the crate name(s)"),
        )
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<InfoCmdArgs> {
    let mut info = None;
    if let Some(m) = matches.subcommand_matches("info") {
        // "$ myapp info" was run
        let names = m
            .get_many::<String>("crate_name")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        info = Some(InfoCmdArgs { names });
    }
    info
}
