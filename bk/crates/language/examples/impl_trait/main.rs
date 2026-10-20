use clap::Parser;
use clap::Subcommand;

mod impl_trait;
mod rpit;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "impl_trait")]
    ImplTrait,
    #[command(name = "rpit")]
    Rpit,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ImplTrait => {
                impl_trait::run();
            }
            Commands::Rpit => {
                rpit::run();
            }
        }
    }
}
