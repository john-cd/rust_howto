use clap::Parser;
use clap::Subcommand;

mod heck;
mod indoc;
mod textwrap;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "heck")]
    Heck,
    #[command(name = "indoc")]
    Indoc,
    #[command(name = "textwrap")]
    Textwrap,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Heck => {
                heck::run();
            }
            Commands::Indoc => {
                indoc::run();
            }
            Commands::Textwrap => {
                textwrap::run();
            }
        }
    }
}
