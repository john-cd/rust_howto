//! Handle the `category_badge` CLI subcommand
//!
//! TODO Reviewed ??-??-2026.

use clap::ArgMatches;
use clap::Command;

use crate::CmdArgs;

/// Builds the `category_badge` subcommand of the CLI user interface
pub(super) fn subcommand_category_badge() -> Command {
    Command::new("category_badge")
        .visible_alias("c")
        .about("Create the markdown for category badge(s), given the category(ies)")
        .display_order(2)
        .arg(
            clap::Arg::new("category")
                .required(true)
                .value_name("CATEGORY") // placeholder for the argument's value in the help message / usage.
                .action(clap::ArgAction::Append)
                .help("Enter the category name(s)"),
        )
}

/// Returns the command line arguments for the `category_badge` subcommand
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("category_badge") {
        let categories = m
            .get_many::<String>("category")
            .unwrap_or_default()
            .map(std::convert::Into::into)
            .collect::<Vec<String>>();
        cmdargs = Some(CmdArgs { args: categories });
    }
    cmdargs
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that multiple category arguments are collected into `CmdArgs`.
    #[test]
    fn parses_multiple_categories() {
        let matches = Command::new("templ")
            .subcommand(subcommand_category_badge())
            .try_get_matches_from(["templ", "category_badge", "development-tools", "wasm"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("category_badge subcommand should be present");

        assert_eq!(
            args.args,
            vec!["development-tools".to_owned(), "wasm".to_owned()]
        );
    }

    /// Checks that the visible alias `c` works as an alias for `category_badge`.
    #[test]
    fn alias_c_is_recognized() {
        let matches = Command::new("templ")
            .subcommand(subcommand_category_badge())
            .try_get_matches_from(["templ", "c", "gui"])
            .expect("arguments should parse");

        let args =
            get_cmd(&matches).expect("category_badge subcommand should be recognized via alias");

        assert_eq!(args.args, vec!["gui".to_owned()]);
    }

    /// Checks that omitting the required category argument fails argument parsing.
    #[test]
    fn missing_category_fails() {
        let result = Command::new("templ")
            .subcommand(subcommand_category_badge())
            .try_get_matches_from(["templ", "category_badge"]);

        assert!(
            result.is_err(),
            "missing category argument should fail parsing"
        );
    }

    /// Checks that `get_cmd` returns `None` when the subcommand was not called.
    #[test]
    fn returns_none_when_subcommand_not_invoked() {
        let matches = Command::new("templ")
            .subcommand(subcommand_category_badge())
            .try_get_matches_from(["templ"])
            .expect("base command should parse");

        assert!(get_cmd(&matches).is_none());
    }
}
