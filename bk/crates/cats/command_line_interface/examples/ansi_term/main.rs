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
                let _ = ansi_term_basic::run();
            }
            Commands::AnsiTermBasic1 => {
                let _ = ansi_term_basic1::run();
            }
            Commands::AnsiTermBasic2 => {
                let _ = ansi_term_basic2::run();
            }
            Commands::Colored => {
                let _ = colored::run();
            }
        }
    }
}
