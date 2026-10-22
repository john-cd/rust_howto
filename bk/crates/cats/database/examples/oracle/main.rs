// TODO Reviewed ??-??-2026
#[cfg(any(
    feature = "oracle",
    all(target_os = "linux", feature = "oracle")
))]
use clap::Parser;
#[cfg(any(
    feature = "oracle",
    all(target_os = "linux", feature = "oracle")
))]
use clap::Subcommand;

#[cfg(feature = "oracle")]
mod diesel_oci;
#[cfg(feature = "oracle")]
mod oracle;
#[cfg(all(target_os = "linux", feature = "oracle"))]
mod sibyl;

/// Command-line arguments.
#[cfg(any(feature = "oracle", all(target_os = "linux", feature = "oracle")))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "oracle", all(target_os = "linux", feature = "oracle")))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `diesel_oci` example.
    #[cfg(feature = "oracle")]
    #[command(name = "diesel_oci")]
    DieselOci,
    /// Runs the `oracle` example.
    #[cfg(feature = "oracle")]
    #[command(name = "oracle")]
    Oracle,
    /// Runs the `sibyl` example.
    #[cfg(all(target_os = "linux", feature = "oracle"))]
    #[command(name = "sibyl")]
    Sibyl,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(
        feature = "oracle",
        all(target_os = "linux", feature = "oracle")
    ))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "oracle")]
                Commands::DieselOci => {
                    diesel_oci::run()?;
                }
                #[cfg(feature = "oracle")]
                Commands::Oracle => {
                    oracle::run().map_err(anyhow::Error::msg)?;
                }
                #[cfg(all(target_os = "linux", feature = "oracle"))]
                Commands::Sibyl => {
                    sibyl::run()?;
                }
            }
        }
    }
    Ok(())
}
