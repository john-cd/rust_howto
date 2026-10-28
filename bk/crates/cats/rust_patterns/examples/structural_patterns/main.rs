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
    match Cli::parse().command {
        Commands::Newtype => newtype::run(),
        Commands::Adapter => adapter::run(),
        Commands::Decorator => decorator::run(),
        Commands::Facade => facade::run(),
        Commands::Composite => composite::run(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
