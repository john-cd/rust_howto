//! Runs one of the `xml` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Parse, deserialize, and write XML with `quick-xml`.
mod quick_xml;

/// Builds or traverses xml graph data while preserving relationships needed by downstream analysis.
/// Workspace and rendering code use this module to obtain a consistent view of the graph.
mod roxmltree;

/// This example demonstrates how to read and write XML data using the
mod xml;

/// Parse a small XML document with `xml5ever` and inspect its DOM tree.
mod xml5ever;

/// Turns xml input into structured syntax data while preserving source locations and reporting
/// malformed input. Analysis and validation code use these routines before applying domain-specific
/// rules.
mod xmlparser;

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

    /// Runs the `quick_xml` example.
    #[command(name = "quick_xml")]
    QuickXml,

    /// Runs the `roxmltree` example.
    #[command(name = "roxmltree")]
    Roxmltree,

    /// Runs the `xml` example.
    #[command(name = "xml")]
    Xml,

    /// Runs the `xml5ever` example.
    #[command(name = "xml5ever")]
    Xml5ever,

    /// Runs the `xmlparser` example.
    #[command(name = "xmlparser")]
    Xmlparser,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::QuickXml => {
                quick_xml::run()?;
            }
            Commands::Roxmltree => {
                roxmltree::run()?;
            }
            Commands::Xml => {
                xml::run()?;
            }
            Commands::Xml5ever => {
                xml5ever::run()?;
            }
            Commands::Xmlparser => {
                xmlparser::run()?;
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
    const SUBCOMMANDS: &[&str] = &["quick_xml", "roxmltree", "xml", "xml5ever", "xmlparser"];

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
