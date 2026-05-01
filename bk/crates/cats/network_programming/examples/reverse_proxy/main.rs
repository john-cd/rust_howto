use clap::Parser;
use clap::Subcommand;

mod ngrok;
#[cfg(target_family = "unix")]
mod pingora;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "ngrok")]
    Ngrok,
    #[cfg(target_family = "unix")]
    #[command(name = "pingora")]
    Pingora,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Ngrok => {
                let _ = ngrok::run();
            }
            #[cfg(target_family = "unix")]
            Commands::Pingora => {
                let _ = pingora::run();
            }
        }
    }
}
