//! Handle the `categories_badges_for_crate` CLI subcommand
//!
//! TODO Reviewed ??-??-2026.

use clap::ArgMatches;
use clap::Command;

use crate::CmdArgs;

/// Builds the `categories_badges_for_crate` subcommand of the CLI user
/// interface
pub(super) fn subcommand_category_badges_for_crate() -> Command {
    Command::new("categories_badges_for_crate")
        .visible_alias("cc")
        .about("Create the markdown for all category badges for (a) given crate(s)")
        .display_order(1)
        .arg(super::arg_crate_name())
}

/// Get the command-line arguments for the `categories_badges_for_crate` subcommand
///
/// # Arguments
/// * `matches` - The matches from the command line arguments
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs> {
    let mut cmdargs = None;
    if let Some(m) = matches.subcommand_matches("categories_badges_for_crate") {
        let names = super::get_cmd_arg_crate_name(m);
        cmdargs = Some(CmdArgs { args: names });
    }
    cmdargs
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that multiple crate names are collected into `CmdArgs`.
    #[test]
    fn parses_multiple_crate_names() {
        let matches = Command::new("templ")
            .subcommand(subcommand_category_badges_for_crate())
            .try_get_matches_from(["templ", "categories_badges_for_crate", "clap", "anyhow"])
            .expect("arguments should parse");

        let args =
            get_cmd(&matches).expect("categories_badges_for_crate subcommand should be present");

        assert_eq!(args.args, vec!["clap".to_owned(), "anyhow".to_owned()]);
    }

    /// Checks that the visible alias `cc` works as an alias for `categories_badges_for_crate`.
    #[test]
    fn alias_cc_is_recognized() {
        let matches = Command::new("templ")
            .subcommand(subcommand_category_badges_for_crate())
            .try_get_matches_from(["templ", "cc", "serde"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("subcommand should be recognized via alias cc");

        assert_eq!(args.args, vec!["serde".to_owned()]);
    }

    /// Checks that omitting the required crate argument fails argument parsing.
    #[test]
    fn missing_crate_name_fails() {
        let result = Command::new("templ")
            .subcommand(subcommand_category_badges_for_crate())
            .try_get_matches_from(["templ", "categories_badges_for_crate"]);

        assert!(result.is_err(), "missing crate name should fail parsing");
    }

    /// Checks that `get_cmd` returns `None` when the subcommand was not called.
    #[test]
    fn returns_none_when_subcommand_not_invoked() {
        let matches = Command::new("templ")
            .subcommand(subcommand_category_badges_for_crate())
            .try_get_matches_from(["templ"])
            .expect("base command should parse");

        assert!(get_cmd(&matches).is_none());
    }
}
