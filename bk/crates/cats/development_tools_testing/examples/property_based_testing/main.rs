use clap::Parser;
use clap::Subcommand;

mod fake;
mod proptest;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "fake")]
    Fake,
    #[command(name = "proptest")]
    Proptest,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Fake => {
                fake::run();
            }
            Commands::Proptest => {
                proptest::run();
            }
        }
    }
}
