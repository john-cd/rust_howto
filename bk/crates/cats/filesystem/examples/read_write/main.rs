use clap::Parser;
use clap::Subcommand;

mod memmap2;
mod read_file;
mod same_file;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "memmap2")]
    Memmap2,
    #[command(name = "read_file")]
    ReadFile,
    #[command(name = "same_file")]
    SameFile,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Memmap2 => {
                memmap2::run();
            }
            Commands::ReadFile => {
                read_file::run();
            }
            Commands::SameFile => {
                same_file::run();
            }
        }
    }
}
