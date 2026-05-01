use clap::Parser;
use clap::Subcommand;

mod subtle;
mod zeroize;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "subtle")]
    Subtle,
    #[command(name = "zeroize")]
    Zeroize,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Subtle => {
                subtle::run();
            }
            Commands::Zeroize => {
                zeroize::run();
            }
        }
    }
}
