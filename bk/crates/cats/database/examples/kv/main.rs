#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
use clap::Parser;
#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
use clap::Subcommand;

#[cfg(feature = "lmdb")]
mod heed;
#[cfg(feature = "rocksdb")]
mod rocksdb;

#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "lmdb")]
    #[command(name = "heed")]
    Heed,
    #[cfg(feature = "rocksdb")]
    #[command(name = "rocksdb")]
    Rocksdb,
}

fn main() {
    #[cfg(any(feature = "lmdb", feature = "rocksdb"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "lmdb")]
                Commands::Heed => {
                    heed::run();
                }
                #[cfg(feature = "rocksdb")]
                Commands::Rocksdb => {
                    rocksdb::run();
                }
            }
        }
    }
}
