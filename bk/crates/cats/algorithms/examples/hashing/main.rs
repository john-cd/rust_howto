use clap::Parser;
use clap::Subcommand;

mod blake3;
mod md5;
mod sha1;
mod sha2;
mod sha_digest;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "blake3")]
    Blake3,
    #[command(name = "md5")]
    Md5,
    #[command(name = "sha1")]
    Sha1,
    #[command(name = "sha2")]
    Sha2,
    #[command(name = "sha_digest")]
    ShaDigest,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Blake3 => {
                blake3::run();
            }
            Commands::Md5 => {
                md5::run();
            }
            Commands::Sha1 => {
                sha1::run();
            }
            Commands::Sha2 => {
                sha2::run()?;
            }
            Commands::ShaDigest => {
                sha_digest::run()?;
            }
        }
    }

    Ok(())
}
