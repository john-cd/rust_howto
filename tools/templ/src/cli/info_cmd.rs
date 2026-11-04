//! The `info` subcommand, which gets information about a crate.
//!
//! TODO Reviewed ??-??-2026.

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that multiple crate names are collected into `CmdArgs`.
    #[test]
    fn parses_multiple_crate_names() {
        let matches = Command::new("templ")
            .subcommand(subcommand_info())
            .try_get_matches_from(["templ", "info", "serde", "tokio"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("info subcommand should be present");

        assert_eq!(args.args, vec!["serde".to_owned(), "tokio".to_owned()]);
    }

    /// Checks that the visible alias `i` works as an alias for `info`.
    #[test]
    fn alias_i_is_recognized() {
        let matches = Command::new("templ")
            .subcommand(subcommand_info())
            .try_get_matches_from(["templ", "i", "axum"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("info subcommand should be recognized via alias i");

        assert_eq!(args.args, vec!["axum".to_owned()]);
    }

    /// Checks that omitting the required crate argument fails argument parsing.
    #[test]
    fn missing_crate_name_fails() {
        let result = Command::new("templ")
            .subcommand(subcommand_info())
            .try_get_matches_from(["templ", "info"]);

        assert!(result.is_err(), "missing crate name should fail parsing");
    }

    /// Checks that `get_cmd` returns `None` when the subcommand was not called.
    #[test]
    fn returns_none_when_subcommand_not_invoked() {
        let matches = Command::new("templ")
            .subcommand(subcommand_info())
            .try_get_matches_from(["templ"])
            .expect("base command should parse");

        assert!(get_cmd(&matches).is_none());
    }
}
