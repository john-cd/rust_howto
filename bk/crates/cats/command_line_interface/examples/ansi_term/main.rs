use clap::Parser;
use clap::Subcommand;

mod ansi_term_basic;
mod ansi_term_basic1;
mod ansi_term_basic2;
mod colored;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "ansi_term_basic")]
    AnsiTermBasic,
    #[command(name = "ansi_term_basic1")]
    AnsiTermBasic1,
    #[command(name = "ansi_term_basic2")]
    AnsiTermBasic2,
    #[command(name = "colored")]
    Colored,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AnsiTermBasic => {
                ansi_term_basic::run();
            }
            Commands::AnsiTermBasic1 => {
                ansi_term_basic1::run();
            }
            Commands::AnsiTermBasic2 => {
                ansi_term_basic2::run();
            }
            Commands::Colored => {
                colored::run();
            }
        }
    }
}
