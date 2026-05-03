use clap::Parser;
use clap::Subcommand;

#[cfg(target_os = "linux")]
mod glommio;
mod listen_unused;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(target_os = "linux")]
    #[command(name = "glommio")]
    Glommio,
    #[command(name = "listen_unused")]
    ListenUnused,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(target_os = "linux")]
            Commands::Glommio => {
                glommio::run();
            }
            Commands::ListenUnused => {
                listen_unused::run();
            }
        }
    }
}
