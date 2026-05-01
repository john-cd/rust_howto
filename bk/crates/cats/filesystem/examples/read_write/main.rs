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
                let _ = memmap2::run();
            }
            Commands::ReadFile => {
                let _ = read_file::run();
            }
            Commands::SameFile => {
                let _ = same_file::run();
            }
        }
    }
}
