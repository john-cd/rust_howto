use clap::Parser;
use clap::Subcommand;

mod argon2;
mod bcrypt;
mod pbkdf2;
mod scrypt;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "argon2")]
    Argon2,
    #[command(name = "bcrypt")]
    Bcrypt,
    #[command(name = "pbkdf2")]
    Pbkdf2,
    #[command(name = "scrypt")]
    Scrypt,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Argon2 => {
                argon2::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::Bcrypt => {
                bcrypt::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Pbkdf2 => {
                pbkdf2::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::Scrypt => {
                scrypt::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
        }
    }

    Ok(())
}
