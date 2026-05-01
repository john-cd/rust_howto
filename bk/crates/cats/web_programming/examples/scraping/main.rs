use clap::Parser;
use clap::Subcommand;

mod broken;
mod extract_links;
mod unique;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "broken")]
    Broken,
    #[command(name = "extract_links")]
    ExtractLinks,
    #[command(name = "unique")]
    Unique,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Broken => {
                let _ = broken::run();
            }
            Commands::ExtractLinks => {
                let _ = extract_links::run();
            }
            Commands::Unique => {
                let _ = unique::run();
            }
        }
    }
}
