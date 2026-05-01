use clap::Parser;
use clap::Subcommand;

mod dyn_clone;
mod pin_project;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "dyn_clone")]
    DynClone,
    #[command(name = "pin_project")]
    PinProject,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DynClone => {
                dyn_clone::run();
            }
            Commands::PinProject => {
                pin_project::run();
            }
        }
    }
}
