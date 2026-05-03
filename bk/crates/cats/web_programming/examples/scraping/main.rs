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
                broken::run();
            }
            Commands::ExtractLinks => {
                extract_links::run();
            }
            Commands::Unique => {
                unique::run();
            }
        }
    }
}
