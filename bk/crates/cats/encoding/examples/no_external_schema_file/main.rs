use clap::Parser;
use clap::Subcommand;

mod postcard;
mod rkyv;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "postcard")]
    Postcard,
    #[command(name = "rkyv")]
    Rkyv,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Postcard => {
                let _ = postcard::run();
            }
            Commands::Rkyv => {
                let _ = rkyv::run();
            }
        }
    }
}
