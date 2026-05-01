use clap::Parser;
use clap::Subcommand;

mod cell;
mod once_cell;
mod rc_refcell;
mod refcell;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cell")]
    Cell,
    #[command(name = "once_cell")]
    OnceCell,
    #[command(name = "rc_refcell")]
    RcRefcell,
    #[command(name = "refcell")]
    Refcell,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cell => {
                let _ = cell::run();
            }
            Commands::OnceCell => {
                let _ = once_cell::run();
            }
            Commands::RcRefcell => {
                let _ = rc_refcell::run();
            }
            Commands::Refcell => {
                let _ = refcell::run();
            }
        }
    }
}
