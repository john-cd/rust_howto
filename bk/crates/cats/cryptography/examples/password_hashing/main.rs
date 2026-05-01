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

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Argon2 => {
                argon2::run();
            }
            Commands::Bcrypt => {
                bcrypt::run();
            }
            Commands::Pbkdf2 => {
                pbkdf2::run();
            }
            Commands::Scrypt => {
                scrypt::run();
            }
        }
    }
}
