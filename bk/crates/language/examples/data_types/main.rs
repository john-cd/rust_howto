use clap::Parser;
use clap::Subcommand;

mod compound_data_types;
mod scalar_data_types;
mod string_data_types;
mod unit_never;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "compound_data_types")]
    CompoundDataTypes,
    #[command(name = "scalar_data_types")]
    ScalarDataTypes,
    #[command(name = "string_data_types")]
    StringDataTypes,
    #[command(name = "unit_never")]
    UnitNever,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CompoundDataTypes => {
                let _ = compound_data_types::run();
            }
            Commands::ScalarDataTypes => {
                let _ = scalar_data_types::run();
            }
            Commands::StringDataTypes => {
                let _ = string_data_types::run();
            }
            Commands::UnitNever => {
                let _ = unit_never::run();
            }
        }
    }
}
