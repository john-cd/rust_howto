use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "arrow")]
mod arrow;
#[cfg(feature = "datafusion")]
mod datafusion;
#[cfg(feature = "polars")]
mod polars;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "arrow")]
    #[command(name = "arrow")]
    Arrow,
    #[cfg(feature = "datafusion")]
    #[command(name = "datafusion")]
    Datafusion,
    #[cfg(feature = "polars")]
    #[command(name = "polars")]
    Polars,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "arrow")]
            Commands::Arrow => arrow::run()?,
            #[cfg(feature = "datafusion")]
            Commands::Datafusion => datafusion::run(),
            #[cfg(feature = "polars")]
            Commands::Polars => polars::run(),
        }
    }

    Ok(())
}
