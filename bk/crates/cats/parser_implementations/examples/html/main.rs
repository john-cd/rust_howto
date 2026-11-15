//! Runs one of the `html` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Turns html input into structured syntax data while preserving source locations and reporting
/// malformed input. Analysis and validation code use these routines before applying domain-specific
/// rules.
mod cssparser;

/// Tokenize HTML with `html5ever` and print the parsed tokens.
mod html5ever;

/// Selects HTML elements and extracts their text with CSS selectors.
mod scraper;

/// This example demonstrates how to use the `tl` crate to parse HTML.
mod tl;

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

    /// Runs the `cssparser` example.
    #[command(name = "cssparser")]
    Cssparser,

    /// Runs the `html5ever` example.
    #[command(name = "html5ever")]
    Html5ever,

    /// Runs the `scraper` CSS selector example.
    #[command(name = "scraper")]
    Scraper,

    /// Runs the `tl` example.
    #[command(name = "tl")]
    Tl,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Cssparser => {
                cssparser::run()?;
            }
            Commands::Html5ever => {
                html5ever::run()?;
            }
            Commands::Scraper => {
                scraper::run()?;
            }
            Commands::Tl => {
                tl::run()?;
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["cssparser", "html5ever", "scraper", "tl"];

    /// Confirms that the command and its arguments are valid.
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
}
