use clap::Parser;
use clap::Subcommand;

mod hyper;
mod ureq;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "hyper")]
    Hyper,
    #[command(name = "ureq")]
    Ureq,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Hyper => {
                hyper::run()?;
            }
            Commands::Ureq => {
                ureq::run()?;
            }
        }
    }

    Ok(())
}
