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
                let _ = any::run();
            }
            Commands::Plugin => {
                let _ = plugin::run();
            }
            Commands::TypeId => {
                let _ = type_id::run();
            }
        }
    }
}
