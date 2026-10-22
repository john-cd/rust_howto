// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

#[cfg(not(windows))] // TODO review Windows support for window creation examples
mod baseview;
mod tao;
mod winit;

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
    /// Runs the `baseview` example.
    #[cfg(not(windows))]
    #[command(name = "baseview")]
    Baseview,
    /// Runs the `tao` example.
    #[command(name = "tao")]
    Tao,
    /// Runs the `winit` example.
    #[command(name = "winit")]
    Winit,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
            Commands::Baseview => {
                baseview::run()?;
            }
            Commands::Tao => {
                tao::run();
            }
            Commands::Winit => {
                winit::run()?;
            }
        }
    }
    Ok(())
}
