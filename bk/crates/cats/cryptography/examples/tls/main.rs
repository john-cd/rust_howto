use clap::Parser;
use clap::Subcommand;

mod native_tls;
mod rustls;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "native_tls")]
    NativeTls,
    #[command(name = "rustls")]
    Rustls,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::NativeTls => {
                native_tls::run();
            }
            Commands::Rustls => {
                rustls::run();
            }
        }
    }
}
