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
    Basic,
    #[command(name = "sort_float")]
    Float,
    #[command(name = "sort_struct")]
    Struct,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Basic => {
                sort_basic::run();
            }
            Commands::Float => {
                sort_float::run();
            }
            Commands::Struct => {
                sort_struct::run();
            }
        }
    }
}
