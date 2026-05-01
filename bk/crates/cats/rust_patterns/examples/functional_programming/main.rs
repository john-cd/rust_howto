use clap::Parser;
use clap::Subcommand;

mod either;
mod frunk;
mod im;
mod itertools;
mod rpds;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "either")]
    Either,
    #[command(name = "frunk")]
    Frunk,
    #[command(name = "im")]
    Im,
    #[command(name = "itertools")]
    Itertools,
    #[command(name = "rpds")]
    Rpds,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Either => {
                either::run();
            }
            Commands::Frunk => {
                frunk::run();
            }
            Commands::Im => {
                im::run();
            }
            Commands::Itertools => {
                itertools::run();
            }
            Commands::Rpds => {
                rpds::run();
            }
        }
    }
}
