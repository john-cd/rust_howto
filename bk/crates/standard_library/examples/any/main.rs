use clap::Parser;
use clap::Subcommand;

mod any;
mod plugin;
mod type_id;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "any")]
    Any,
    #[command(name = "plugin")]
    Plugin,
    #[command(name = "type_id")]
    TypeId,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Any => {
                any::run();
            }
            Commands::Plugin => {
                plugin::run();
            }
            Commands::TypeId => {
                type_id::run();
            }
        }
    }
}
