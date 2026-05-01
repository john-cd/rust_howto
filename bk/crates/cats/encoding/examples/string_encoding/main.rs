use clap::Parser;
use clap::Subcommand;

mod base64;
mod hex;
mod percent_encode;
mod percent_encoding;
mod url_encode;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "base64")]
    Base64,
    #[command(name = "hex")]
    Hex,
    #[command(name = "percent_encode")]
    PercentEncode,
    #[command(name = "percent_encoding")]
    PercentEncoding,
    #[command(name = "url_encode")]
    UrlEncode,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Base64 => {
                let _ = base64::run();
            }
            Commands::Hex => {
                let _ = hex::run();
            }
            Commands::PercentEncode => {
                let _ = percent_encode::run();
            }
            Commands::PercentEncoding => {
                let _ = percent_encoding::run();
            }
            Commands::UrlEncode => {
                let _ = url_encode::run();
            }
        }
    }
}
