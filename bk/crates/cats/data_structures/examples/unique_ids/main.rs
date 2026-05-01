use clap::Parser;
use clap::Subcommand;

mod ulid;
mod uuid;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "ulid")]
    Ulid,
    #[command(name = "uuid")]
    Uuid,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Ulid => {
                let _ = ulid::run();
            }
            Commands::Uuid => {
                let _ = uuid::run();
            }
        }
    }
}
