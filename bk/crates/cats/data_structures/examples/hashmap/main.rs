use clap::Parser;
use clap::Subcommand;

mod custom_hash_function;
mod custom_type_as_key;
mod hashmap;
mod hashset;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "custom_hash_function")]
    CustomHashFunction,
    #[command(name = "custom_type_as_key")]
    CustomTypeAsKey,
    #[command(name = "hashmap")]
    Hashmap,
    #[command(name = "hashset")]
    Hashset,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CustomHashFunction => {
                custom_hash_function::run();
            }
            Commands::CustomTypeAsKey => {
                custom_type_as_key::run();
            }
            Commands::Hashmap => {
                hashmap::run();
            }
            Commands::Hashset => {
                hashset::run();
            }
        }
    }
}
