use clap::Parser;
use clap::Subcommand;

mod btreemap;
mod btreeset;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "btreemap")]
    Btreemap,
    #[command(name = "btreeset")]
    Btreeset,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Btreemap => {
                btreemap::run();
            }
            Commands::Btreeset => {
                btreeset::run();
            }
        }
    }
}
