use clap::Parser;
use clap::Subcommand;

mod directories;
mod dirs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "directories")]
    Directories,
    #[command(name = "dirs")]
    Dirs,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Directories => {
                let _ = directories::run();
            }
            Commands::Dirs => {
                let _ = dirs::run();
            }
        }
    }
}
