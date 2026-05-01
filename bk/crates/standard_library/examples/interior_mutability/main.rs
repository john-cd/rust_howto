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
                cell::run();
            }
            Commands::OnceCell => {
                once_cell::run();
            }
            Commands::RcRefcell => {
                rc_refcell::run();
            }
            Commands::Refcell => {
                refcell::run();
            }
        }
    }
}
