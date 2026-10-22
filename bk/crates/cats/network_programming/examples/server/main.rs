// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

#[cfg(target_os = "linux")]
mod glommio;
mod listen_unused;

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
    /// Runs the `glommio` example.
    #[cfg(target_os = "linux")]
    #[command(name = "glommio")]
    Glommio,
    /// Runs the `listen_unused` example.
    #[command(name = "listen_unused")]
    ListenUnused,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(target_os = "linux")]
            Commands::Glommio => {
                glommio::run();
            }
            Commands::ListenUnused => {
                listen_unused::run()?;
            }
        }
    }

    Ok(())
}
