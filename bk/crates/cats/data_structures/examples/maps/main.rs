use clap::Parser;
use clap::Subcommand;

mod indexmap;
mod linked_hash_map;
mod multimap;
mod slotmap;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "indexmap")]
    Indexmap,
    #[command(name = "linked_hash_map")]
    LinkedHashMap,
    #[command(name = "multimap")]
    Multimap,
    #[command(name = "slotmap")]
    Slotmap,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Indexmap => {
                indexmap::run();
            }
            Commands::LinkedHashMap => {
                linked_hash_map::run();
            }
            Commands::Multimap => {
                multimap::run();
            }
            Commands::Slotmap => {
                slotmap::run();
            }
        }
    }
}
