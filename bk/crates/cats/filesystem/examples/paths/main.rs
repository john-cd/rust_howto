use clap::Parser;
use clap::Subcommand;

mod camino;
mod canonicalize;
mod manipulate_paths;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "camino")]
    Camino,
    #[command(name = "canonicalize")]
    Canonicalize,
    #[command(name = "manipulate_paths")]
    ManipulatePaths,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Camino => {
                camino::run();
            }
            Commands::Canonicalize => {
                canonicalize::run();
            }
            Commands::ManipulatePaths => {
                manipulate_paths::run();
            }
        }
    }
}
