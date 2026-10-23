//! Runs one of the `xml` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod quick_xml;
mod roxmltree;
mod xml;
mod xml5ever;
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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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
                xml5ever::run();
            }
            Commands::Xmlparser => {
                xmlparser::run();
            }
        }
    }
    Ok(())
}
