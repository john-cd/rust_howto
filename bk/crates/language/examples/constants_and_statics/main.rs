use clap::Parser;
use clap::Subcommand;

mod constants;
mod static_mut;
mod statics;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "constants")]
    Constants,
    #[command(name = "static_mut")]
    StaticMut,
    #[command(name = "statics")]
    Statics,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Constants => {
                constants::run();
            }
            Commands::StaticMut => {
                static_mut::run();
            }
            Commands::Statics => {
                statics::run();
            }
        }
    }
}
