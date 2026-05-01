use clap::Parser;
use clap::Subcommand;

mod cached;
mod lru;
mod moka;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cached")]
    Cached,
    #[command(name = "lru")]
    Lru,
    #[command(name = "moka")]
    Moka,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cached => {
                let _ = cached::run();
            }
            Commands::Lru => {
                let _ = lru::run();
            }
            Commands::Moka => {
                let _ = moka::run();
            }
        }
    }
}
