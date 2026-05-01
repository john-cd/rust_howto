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
                let _ = heck::run();
            }
            Commands::Indoc => {
                let _ = indoc::run();
            }
            Commands::Textwrap => {
                let _ = textwrap::run();
            }
        }
    }
}
