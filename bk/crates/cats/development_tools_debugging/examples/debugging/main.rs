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
fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Commands::DbgMacro => dbg_macro::run(),
        Commands::Backtrace => backtrace::run(),
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
