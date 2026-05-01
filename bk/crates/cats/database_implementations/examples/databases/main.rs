#[cfg(any(feature = "sled", feature = "surrealdb"))]
use clap::Parser;
#[cfg(any(feature = "sled", feature = "surrealdb"))]
use clap::Subcommand;

#[cfg(feature = "sled")]
mod sled;
#[cfg(feature = "surrealdb")]
mod surrealdb;

#[cfg(any(feature = "sled", feature = "surrealdb"))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(any(feature = "sled", feature = "surrealdb"))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "sled")]
    #[command(name = "sled")]
    Sled,
    #[cfg(feature = "surrealdb")]
    #[command(name = "surrealdb")]
    Surrealdb,
}

fn main() {
    #[cfg(any(feature = "sled", feature = "surrealdb"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "sled")]
                Commands::Sled => {
                    let _ = sled::run();
                }
                #[cfg(feature = "surrealdb")]
                Commands::Surrealdb => {
                    let _ = surrealdb::run();
                }
            }
        }
    }
}
