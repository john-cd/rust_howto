use clap::Parser;
use clap::Subcommand;

mod sort_basic;
mod sort_float;
mod sort_struct;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "sort_basic")]
    SortBasic,
    #[command(name = "sort_float")]
    SortFloat,
    #[command(name = "sort_struct")]
    SortStruct,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::SortBasic => {
                sort_basic::run();
            }
            Commands::SortFloat => {
                sort_float::run();
            }
            Commands::SortStruct => {
                sort_struct::run();
            }
        }
    }
}
