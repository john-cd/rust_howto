//! Runs examples of structural patterns in Rust.

use clap::Parser;
use clap::Subcommand;

mod adapter;
mod composite;
mod decorator;
mod facade;
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

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// Every subcommand is parsed and runs its example.
    #[test]
    fn every_subcommand_dispatches() {
        let names: Vec<String> = Cli::command()
            .get_subcommands()
            .map(|c| c.get_name().to_owned())
            .collect();
        assert!(!names.is_empty());
        for name in names {
            let cli = Cli::try_parse_from(["example", name.as_str()])
                .expect("the subcommand should parse");
            dispatch(cli).expect("the example should run");
        }
    }

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
