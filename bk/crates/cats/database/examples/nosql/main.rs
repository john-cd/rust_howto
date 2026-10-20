#[cfg(any(feature = "mongodb", feature = "redis"))]
use clap::Parser;
#[cfg(any(feature = "mongodb", feature = "redis"))]
use clap::Subcommand;

#[cfg(feature = "mongodb")]
mod mongodb;
#[cfg(feature = "redis")]
mod redis;

#[cfg(any(feature = "mongodb", feature = "redis"))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(any(feature = "mongodb", feature = "redis"))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "mongodb")]
    #[command(name = "mongodb")]
    Mongodb,
    #[cfg(feature = "redis")]
    #[command(name = "redis")]
    Redis,
}

fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "mongodb", feature = "redis"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "mongodb")]
                Commands::Mongodb => {
                    mongodb::run()?;
                }
                #[cfg(feature = "redis")]
                Commands::Redis => {
                    redis::run()?;
                }
            }
        }
    }
    Ok(())
}
