//! Handle the `badge` CLI subcommand

use std::path::PathBuf;

use clap::ArgMatches;
use clap::Command;
use clap_builder::builder::ValueHint;

use crate::CmdArgs2;

/// Builds the `badge` subcommand of the CLI user interface.
///
/// This function defines the structure and options for the `badge` subcommand,
/// which is used to create markdown for crate badges.
///
/// The subcommand supports the following:
/// - An alias `b` for quick access.
/// - A description: "Create the markdown for (a) crate badge(s)".
/// - A display order of `0` to prioritize its visibility in help messages.
/// - An argument for specifying the crate name (defined in `super::arg_crate_name()`).
/// - An optional argument `-a` or `--add` to specify a file to add the generated
///   reference definitions to.
///
/// The `-a` or `--add` argument has the following characteristics:
/// - It is optional (`required(false)`).
/// - It can be set with or without an equals sign (`-a=/path/to/file` or `-a /path/to/file`).
/// - It can take zero or one argument (`num_args(0..=1)`).
/// - If no value is provided, it defaults to `"./src/refs/crate-refs.md"` (`default_missing_value`).
/// - It expects a file path (`value_parser(clap::value_parser!(PathBuf))`).
/// - It provides a file path hint to the shell for autocompletion (`value_hint(ValueHint::FilePath)`).
/// - It has a help message: "Add the newly created refdefs to the master markdown file containing reference definitions (crate-refs.md by default), if they don't exist already".
///
/// # Returns
///
/// A `clap::Command` representing the configured `badge` subcommand.
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
            .default_missing_value("./src/refs/crate-refs.md")
            .value_name("REFDEF_FILE")
            .value_parser(clap::value_parser!(PathBuf))
            .value_hint(ValueHint::FilePath)
            .help("Add the newly created refdefs to the master markdown file containing reference definitions (crate-refs.md by default), if they don't exist already"))
}
// fix hardcoded path

/// Extracts the command arguments for the `badge` subcommand.
///
/// This function processes the command-line arguments provided to the `badge`
/// subcommand and extracts the relevant information.
///
/// # Arguments
///
/// * `matches` - A reference to the `clap::ArgMatches` containing the parsed
///   command-line arguments.
///
/// # Returns
///
/// An `Option<CmdArgs2>` containing the extracted arguments, or `None` if the
/// `badge` subcommand was not invoked.
///
/// The `CmdArgs2` struct contains:
/// - `args`: A `Vec<String>` of crate names.
/// - `file`: An `Option<PathBuf>` representing the path to the reference
///   definition file, if provided.
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
