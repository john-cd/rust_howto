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

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dsa => {
                let _ = dsa::run();
            }
            Commands::Ecdsa => {
                let _ = ecdsa::run();
            }
            Commands::Ed25519 => {
                let _ = ed25519::run();
            }
            Commands::Ed25519Dalek => {
                let _ = ed25519_dalek::run();
            }
            Commands::Hmac => {
                let _ = hmac::run();
            }
        }
    }
}
