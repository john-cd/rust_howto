use clap::Parser;
use clap::Subcommand;

mod from;
mod from2;
mod from3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "from")]
    From,
    #[command(name = "from2")]
    From2,
    #[command(name = "from3")]
    From3,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::From => {
                let _ = from::run();
            }
            Commands::From2 => {
                let _ = from2::run();
            }
            Commands::From3 => {
                let _ = from3::run();
            }
        }
    }
}
