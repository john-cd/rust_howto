//! Runs the small debugger and backtrace examples.

use clap::Parser;
use clap::Subcommand;

mod backtrace;
mod dbg_macro;

/// Command-line arguments for the debugging examples.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Commands,
}

/// The debugging examples available from this target.
#[derive(Subcommand)]
enum Commands {
    /// Prints and returns a value with `dbg!`.
    DbgMacro,
    /// Captures and prints a standard-library backtrace.
    Backtrace,
}

/// Runs the selected debugging example.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::DbgMacro => dbg_macro::run(),
        Commands::Backtrace => backtrace::run(),
    }
}

fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each example is reachable through its own subcommand and runs.
    #[test]
    fn test_every_subcommand_parses_and_runs() -> anyhow::Result<()> {
        for name in ["dbg-macro", "backtrace"] {
            let cli = Cli::try_parse_from(["example", name])?;
            dispatch(cli)?;
        }
        Ok(())
    }

    /// A subcommand is required.
    #[test]
    fn test_subcommand_is_required() {
        assert!(Cli::try_parse_from(["example"]).is_err());
    }
}
