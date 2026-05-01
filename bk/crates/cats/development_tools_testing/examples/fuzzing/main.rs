use clap::Parser;
use clap::Subcommand;

mod afl;
mod bolero_example;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "afl")]
    Afl,
    #[command(name = "bolero_example")]
    BoleroExample,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Afl => {
                let _ = afl::run();
            }
            Commands::BoleroExample => {
                let _ = bolero_example::run();
            }
        }
    }
}
