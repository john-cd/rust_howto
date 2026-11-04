//! Handle the `badge` CLI subcommand
//!
//! TODO Reviewed ??-??-2026.

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
/// - An optional argument `-a` or `--add` to specify a file to add the generated reference
///   definitions to.
///
/// The `-a` or `--add` argument has the following characteristics:
/// - It is optional (`required(false)`).
/// - It accepts a reference-definition file path with or without an equals sign (`-a /path/to/file`
///   or `-a=/path/to/file`).
/// - It takes exactly one argument when provided (`num_args(1)`).
/// - It expects a file path (`value_parser(clap::value_parser!(PathBuf))`).
/// - It provides a file path hint to the shell for autocompletion
///   (`value_hint(ValueHint::FilePath)`).
/// - It has a help message explaining that the generated reference definitions are merged into the
///   specified file if they do not exist already.
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
            .num_args(1)
            .value_name("REFDEF_FILE")
            .value_parser(clap::value_parser!(PathBuf))
            .value_hint(ValueHint::FilePath)
            .help("Merge newly created reference definitions into this file, if they do not exist already"))
}

/// Extracts the command arguments for the `badge` subcommand.
///
/// This function processes the command-line arguments provided to the `badge`
/// subcommand and extracts the relevant information.
///
/// # Arguments
///
/// * `matches` - A reference to the `clap::ArgMatches` containing the parsed command-line
///   arguments.
///
/// # Returns
///
/// An `Option<CmdArgs2>` containing the extracted arguments, or `None` if the
/// `badge` subcommand was not invoked.
///
/// The `CmdArgs2` struct contains:
/// - `args`: A `Vec<String>` of crate names.
/// - `file`: An `Option<PathBuf>` representing the path to the reference definition file, if
///   provided.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The badge command preserves a caller-supplied reference-definition path.
    #[test]
    fn add_option_uses_the_path_supplied_by_the_caller() {
        let matches = Command::new("templ")
            .subcommand(subcommand_badge())
            .try_get_matches_from(["templ", "badge", "serde", "--add", "custom/refs.md"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("badge subcommand should be present");

        assert_eq!(args.file, Some(PathBuf::from("custom/refs.md")));
    }

    /// Verifies short `-a` option correctly propagates the reference file path.
    #[test]
    fn add_short_option_propagates_path() {
        let matches = Command::new("templ")
            .subcommand(subcommand_badge())
            .try_get_matches_from(["templ", "badge", "tokio", "-a", "refs.md"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("badge subcommand should be present");

        assert_eq!(args.args, vec!["tokio".to_owned()]);
        assert_eq!(args.file, Some(PathBuf::from("refs.md")));
    }

    /// Checks that multiple crate names are collected and the add option defaults to None when omitted.
    #[test]
    fn multiple_crate_names_without_add_option() {
        let matches = Command::new("templ")
            .subcommand(subcommand_badge())
            .try_get_matches_from(["templ", "badge", "serde", "tokio", "axum"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("badge subcommand should be present");

        assert_eq!(
            args.args,
            vec!["serde".to_owned(), "tokio".to_owned(), "axum".to_owned()]
        );
        assert_eq!(args.file, None);
    }

    /// Checks that the visible alias `b` works as an alias for `badge`.
    #[test]
    fn alias_b_is_recognized() {
        let matches = Command::new("templ")
            .subcommand(subcommand_badge())
            .try_get_matches_from(["templ", "b", "serde"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("badge subcommand should be recognized via alias");

        assert_eq!(args.args, vec!["serde".to_owned()]);
        assert_eq!(args.file, None);
    }

    /// Checks that omitting the required crate name argument fails argument parsing.
    #[test]
    fn missing_crate_name_fails() {
        let result = Command::new("templ")
            .subcommand(subcommand_badge())
            .try_get_matches_from(["templ", "badge"]);

        assert!(result.is_err(), "missing crate name should fail parsing");
    }

    /// Checks that `get_cmd` returns `None` when the badge subcommand was not called.
    #[test]
    fn returns_none_when_subcommand_not_invoked() {
        let matches = Command::new("templ")
            .subcommand(subcommand_badge())
            .try_get_matches_from(["templ"])
            .expect("base command should parse");

        assert!(get_cmd(&matches).is_none());
    }
}
