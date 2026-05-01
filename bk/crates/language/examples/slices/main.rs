use clap::Parser;
use clap::Subcommand;

mod slice_as_argument;
mod slices;
mod string_slices;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "slice_as_argument")]
    SliceAsArgument,
    #[command(name = "slices")]
    Slices,
    #[command(name = "string_slices")]
    StringSlices,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::SliceAsArgument => {
                slice_as_argument::run();
            }
            Commands::Slices => {
                slices::run();
            }
            Commands::StringSlices => {
                string_slices::run();
            }
        }
    }
}
