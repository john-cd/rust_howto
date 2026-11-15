//! Runs examples of structural patterns in Rust.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Adapts buffered input lines into simple comma-separated row values.
mod adapter;

/// Represents and evaluates a tree with leaf and group nodes.
mod composite;

/// Wraps a reader to add byte-counting behavior.
mod decorator;

/// Hides a hash map behind a user-directory API.
mod facade;

/// Uses a newtype so user identifiers cannot be confused with arbitrary
mod newtype;

/// Command-line arguments for the structural pattern examples.
#[derive(Parser)]
struct Cli {

    /// The example to run.
    #[command(subcommand)]
    command: Commands,
}

/// The structural pattern examples available from this target.
#[derive(Subcommand)]
enum Commands {

    /// Demonstrates a newtype domain identifier.
    Newtype,

    /// Demonstrates adapting a buffered reader into an iterator.
    Adapter,

    /// Demonstrates a reader decorator that counts bytes.
    Decorator,

    /// Demonstrates a facade over a user directory.
    Facade,

    /// Demonstrates a recursive composite enum.
    Composite,
}

/// Runs the selected structural pattern example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Newtype => newtype::run(),
        Commands::Adapter => adapter::run(),
        Commands::Decorator => decorator::run(),
        Commands::Facade => facade::run(),
        Commands::Composite => composite::run(),
    }
}

/// Every subcommand is parsed and runs its example.
#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// An unknown subcommand is rejected by the parser.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// The subcommand is required.
    #[test]
    fn subcommand_is_required() {
        assert!(Cli::try_parse_from(["example"]).is_err());
    }
}
