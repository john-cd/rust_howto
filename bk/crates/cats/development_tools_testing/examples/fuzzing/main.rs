use clap::Parser;
use clap::Subcommand;

#[cfg(not(windows))]
mod afl;
mod bolero_example;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(not(windows))]
    #[command(name = "afl")]
    Afl,
    #[command(name = "bolero_example")]
    BoleroExample,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
            Commands::Afl => {
                afl::run();
            }
            Commands::BoleroExample => {
                bolero_example::run();
            }
        }
    }
}
