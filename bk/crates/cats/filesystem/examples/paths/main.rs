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
                let _ = camino::run();
            }
            Commands::Canonicalize => {
                let _ = canonicalize::run();
            }
            Commands::ManipulatePaths => {
                let _ = manipulate_paths::run();
            }
        }
    }
}
