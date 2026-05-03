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
        if let Err(e) = match command {
            Commands::Base => {
                base::run()
            }
            Commands::Fragment => {
                fragment::run()
            }
            Commands::New => {
                new::run()
            }
            Commands::Origin => {
                origin::run()
            }
            Commands::Origin1 => {
                origin1::run()
            }
            Commands::Parse => {
                parse::run()
            }
        }
        {
            eprintln!("{e}");
        }
    }
}
