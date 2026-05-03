use clap::Parser;
use clap::Subcommand;

mod dsa;
mod ecdsa;
mod ed25519;
mod ed25519_dalek;
mod hmac;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "dsa")]
    Dsa,
    #[command(name = "ecdsa")]
    Ecdsa,
    #[command(name = "ed25519")]
    Ed25519,
    #[command(name = "ed25519_dalek")]
    Ed25519Dalek,
    #[command(name = "hmac")]
    Hmac,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dsa => {
                dsa::run();
            }
            Commands::Ecdsa => {
                ecdsa::run()?;
            }
            Commands::Ed25519 => {
                ed25519::run()?;
            }
            Commands::Ed25519Dalek => {
                ed25519_dalek::run()?;
            }
            Commands::Hmac => {
                hmac::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
        }
    }

    Ok(())
}
