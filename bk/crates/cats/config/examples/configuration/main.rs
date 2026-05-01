//! Configuration management examples.

use clap::Parser;
use clap::Subcommand;

mod config_hierarchical;
mod config_singleton;
mod config_testing;
mod confy;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "config_hierarchical")]
    ConfigHierarchical,
    #[command(name = "config_singleton")]
    ConfigSingleton,
    #[command(name = "config_testing")]
    ConfigTesting,
    #[command(name = "confy")]
    Confy,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ConfigHierarchical => {
                config_hierarchical::run();
            }
            Commands::ConfigSingleton => {
                config_singleton::run();
            }
            Commands::ConfigTesting => {
                config_testing::run();
            }
            Commands::Confy => {
                confy::run();
            }
        }
    }
}
