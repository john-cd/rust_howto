use clap::Parser;
use clap::Subcommand;

mod base;
mod fragment;
mod new;
mod origin;
mod origin1;
mod parse;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "base")]
    Base,
    #[command(name = "fragment")]
    Fragment,
    #[command(name = "new")]
    New,
    #[command(name = "origin")]
    Origin,
    #[command(name = "origin1")]
    Origin1,
    #[command(name = "parse")]
    Parse,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Base => {
                let _ = base::run();
            }
            Commands::Fragment => {
                let _ = fragment::run();
            }
            Commands::New => {
                let _ = new::run();
            }
            Commands::Origin => {
                let _ = origin::run();
            }
            Commands::Origin1 => {
                let _ = origin1::run();
            }
            Commands::Parse => {
                let _ = parse::run();
            }
        }
    }
}
