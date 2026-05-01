use clap::Parser;
use clap::Subcommand;

mod result;
mod result2;
mod result3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "result")]
    Result,
    #[command(name = "result2")]
    Result2,
    #[command(name = "result3")]
    Result3,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Result => {
                let _ = result::run();
            }
            Commands::Result2 => {
                let _ = result2::run();
            }
            Commands::Result3 => {
                let _ = result3::run();
            }
        }
    }
}
