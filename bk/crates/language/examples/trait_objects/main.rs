use clap::Parser;
use clap::Subcommand;

mod dyn_autotraits;
mod dyn_compat;
mod dyn_supertraits;
mod trait_objects;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "dyn_autotraits")]
    DynAutotraits,
    #[command(name = "dyn_compat")]
    DynCompat,
    #[command(name = "dyn_supertraits")]
    DynSupertraits,
    #[command(name = "trait_objects")]
    TraitObjects,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DynAutotraits => {
                dyn_autotraits::run();
            }
            Commands::DynCompat => {
                dyn_compat::run();
            }
            Commands::DynSupertraits => {
                dyn_supertraits::run();
            }
            Commands::TraitObjects => {
                trait_objects::run();
            }
        }
    }
}
