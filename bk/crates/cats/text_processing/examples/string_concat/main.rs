use clap::Parser;
use clap::Subcommand;

mod string_concat;
mod string_concat2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "string_concat")]
    StringConcat,
    #[command(name = "string_concat2")]
    StringConcat2,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::StringConcat => {
                string_concat::run();
            }
            Commands::StringConcat2 => {
                string_concat2::run();
            }
        }
    }
}
