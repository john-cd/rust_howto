//! Runs one of the `robotics_frameworks` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

// This dispatcher remains as source but is disabled in Cargo.toml until Zenoh
// and OpenRR have compatible fixes for their affected dependency chains.

use clap::Parser;
use clap::Subcommand;

mod openrr;
#[cfg(feature = "zenoh")]
mod zenoh;

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
    /// Runs the `openrr` example.
    #[command(name = "openrr")]
    Openrr,
    /// Runs the `zenoh` example.
    #[cfg(feature = "zenoh")]
    #[command(name = "zenoh")]
    Zenoh,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Openrr => {
                openrr::run();
            }
            #[cfg(feature = "zenoh")]
            Commands::Zenoh => {
                zenoh::run()?;
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
