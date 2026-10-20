use clap::Parser;
use clap::Subcommand;

mod abstract_factory;
mod factory;
mod singleton;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "abstract_factory")]
    AbstractFactory,
    #[command(name = "factory")]
    Factory,
    #[command(name = "singleton")]
    Singleton,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AbstractFactory => {
                abstract_factory::run();
            }
            Commands::Factory => {
                factory::run();
            }
            Commands::Singleton => {
                singleton::run();
            }
        }
    }
}
