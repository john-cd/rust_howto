use clap::Parser;
use clap::Subcommand;

#[cfg(not(windows))] // TODO review Windows support for Ruby FFI examples
mod magnus;
#[cfg(not(windows))]
mod rutie;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(not(windows))]
    #[command(name = "magnus")]
    Magnus,
    #[cfg(not(windows))]
    #[command(name = "rutie")]
    Rutie,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
            Commands::Magnus => {
                magnus::run();
            }
            #[cfg(not(windows))]
            Commands::Rutie => {
                rutie::run();
            }
        }
    }
}
