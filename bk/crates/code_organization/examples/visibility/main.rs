//! Runs one of the `visibility` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod private_access;
mod pub_keyword;
mod public_access;
mod public_by_default;
mod visibility_external_code;
mod visibility_pub_crate;
mod visibility_scope;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `private_access` example.
    #[command(name = "private_access")]
    PrivateAccess,
    /// Runs the `pub_keyword` example.
    #[command(name = "pub_keyword")]
    PubKeyword,
    /// Runs the `public_access` example.
    #[command(name = "public_access")]
    PublicAccess,
    /// Runs the `public_by_default` example.
    #[command(name = "public_by_default")]
    PublicByDefault,
    /// Runs the `visibility_external_code` example.
    #[command(name = "visibility_external_code")]
    VisibilityExternalCode,
    /// Runs the `visibility_pub_crate` example.
    #[command(name = "visibility_pub_crate")]
    VisibilityPubCrate,
    /// Runs the `visibility_scope` example.
    #[command(name = "visibility_scope")]
    VisibilityScope,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::PrivateAccess => {
                private_access::run();
            }
            Commands::PubKeyword => {
                pub_keyword::run();
            }
            Commands::PublicAccess => {
                public_access::run();
            }
            Commands::PublicByDefault => {
                public_by_default::run();
            }
            Commands::VisibilityExternalCode => {
                visibility_external_code::run();
            }
            Commands::VisibilityPubCrate => {
                visibility_pub_crate::run();
            }
            Commands::VisibilityScope => {
                visibility_scope::run();
            }
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "private_access",
        "pub_keyword",
        "public_access",
        "public_by_default",
        "visibility_external_code",
        "visibility_pub_crate",
        "visibility_scope",
    ];

    /// The command-line definition is internally consistent.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
