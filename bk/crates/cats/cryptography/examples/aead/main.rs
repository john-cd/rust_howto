use clap::Parser;
use clap::Subcommand;

mod aes_gcm;
mod aes_gcm_siv;
mod chacha20poly1305;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "aes_gcm")]
    AesGcm,
    #[command(name = "aes_gcm_siv")]
    AesGcmSiv,
    #[command(name = "chacha20poly1305")]
    Chacha20poly1305,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AesGcm => {
                aes_gcm::run();
            }
            Commands::AesGcmSiv => {
                aes_gcm_siv::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::Chacha20poly1305 => {
                chacha20poly1305::run();
            }
        }
    }

    Ok(())
}
