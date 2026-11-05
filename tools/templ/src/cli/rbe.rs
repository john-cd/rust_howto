//! Handle the `rbe` CLI subcommand
//!
//! TODO Reviewed ??-??-2026.

use clap::Arg;
use clap::ArgMatches;
use clap::Command;

use crate::CmdArgs;

/// Builds the `rbe` subcommand of the CLI user interface
pub(super) fn subcommand_rbe() -> Command {
    Command::new("rbe")
        //.visible_alias("")
        .about("Create the markdown for (a) Rust By Example book badge(s)")
        .display_order(3)
        .arg(
            Arg::new("concept")
                .required(true)
                .value_name("CONCEPT") // placeholder for the argument's value in the help message / usage.
                //.value_parser(value_parser!(...))
                .action(clap::ArgAction::Append)
                .help("Enter the RBE book chapter name e.g. \"attributes\""),
        )
}

/// Get the command line arguments for the `rbe` subcommand.
///
/// Returns `Some(CmdArgs)` if the `rbe` subcommand was used, `None` otherwise.
pub(super) fn get_cmd(matches: &ArgMatches) -> Option<CmdArgs> {
    let mut badge = None;
    if let Some(m) = matches.subcommand_matches("rbe") {
        // "$ myapp rbe" was run
        let concepts = m
            .get_many::<String>("concept")
            .unwrap_or_default()
            .map(std::convert::Into::into)
            .collect::<Vec<String>>();
        badge = Some(CmdArgs { args: concepts });
    }
    badge
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that multiple concepts are collected into `CmdArgs`.
    #[test]
    fn parses_multiple_concepts() {
        let matches = Command::new("templ")
            .subcommand(subcommand_rbe())
            .try_get_matches_from(["templ", "rbe", "attributes", "traits", "generics"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("rbe subcommand should be present");

        assert_eq!(
            args.args,
            vec![
                "attributes".to_owned(),
                "traits".to_owned(),
                "generics".to_owned()
            ]
        );
    }

    /// Checks that a single concept is collected into `CmdArgs`.
    #[test]
    fn parses_single_concept() {
        let matches = Command::new("templ")
            .subcommand(subcommand_rbe())
            .try_get_matches_from(["templ", "rbe", "error_handling"])
            .expect("arguments should parse");

        let args = get_cmd(&matches).expect("rbe subcommand should be present");

        assert_eq!(args.args, vec!["error_handling".to_owned()]);
    }

    /// Checks that omitting the required concept argument fails argument parsing.
    #[test]
    fn missing_concept_fails() {
        let result = Command::new("templ")
            .subcommand(subcommand_rbe())
            .try_get_matches_from(["templ", "rbe"]);

        assert!(result.is_err(), "missing concept should fail parsing");
    }

    /// Checks that `get_cmd` returns `None` when the subcommand was not called.
    #[test]
    fn returns_none_when_subcommand_not_invoked() {
        let matches = Command::new("templ")
            .subcommand(subcommand_rbe())
            .try_get_matches_from(["templ"])
            .expect("base command should parse");

        assert!(get_cmd(&matches).is_none());
    }
}
