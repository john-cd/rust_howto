//! Handle the `badge` CLI subcommand

use std::path::PathBuf;

use clap::ArgMatches;
use clap::Command;
use clap_builder::builder::ValueHint;

use crate::CmdArgs2;

/// Builds the `badge` subcommand of the CLI user interface
pub(super) fn subcommand_badge() -> Command {
    Command::new("badge")
        .visible_alias("b")
        .about("Create the markdown for (a) crate badge(s)")
        .display_order(0)
        .arg(super::arg_crate_name())
        .arg(clap::Arg::new("add")  // -a --add -a=/path/to/file --add=/path/to/file
            .short('a')
            .long("add")
            .required(false)
            .action(clap::ArgAction::Set)
            .num_args(0..=1)
            .require_equals(true)
            .default_missing_value("./src/refs/crate-refs.md") // TODO fix hardcoded path
            .value_name("REFDEF_FILE")
            .value_parser(clap::value_parser!(PathBuf))
            .value_hint(ValueHint::FilePath)
            .help("Add the newly created refdefs to the master markdown file containing reference definitions (crate-refs.md by default), if they don't exist already"))
}

pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs2> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("badge") {
        // "$ myapp badge" was run
        let names = super::get_cmd_arg_crate_name(m);
        let refdef_file: Option<PathBuf> = m.get_one("add").cloned();
        cmdargs = Some(CmdArgs2 {
            args: names,
            file: refdef_file,
        });
    }
    cmdargs
}
