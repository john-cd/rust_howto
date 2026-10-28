//! Runs one of the `reverse_proxy` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod ngrok;
#[cfg(target_family = "unix")]
mod pingora;

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
    /// Runs the `ngrok` example.
    #[command(name = "ngrok")]
    Ngrok,
    /// Runs the `pingora` example.
    #[cfg(target_family = "unix")]
    #[command(name = "pingora")]
    Pingora,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Ngrok => {
                ngrok::run()?;
            }
            #[cfg(target_family = "unix")]
            Commands::Pingora => {
                pingora::run()?;
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
