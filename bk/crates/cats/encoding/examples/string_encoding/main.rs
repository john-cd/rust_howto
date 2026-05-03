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
                base64::run();
            }
            Commands::Hex => {
                hex::run();
            }
            Commands::PercentEncode => {
                percent_encode::run();
            }
            Commands::PercentEncoding => {
                percent_encoding::run();
            }
            Commands::UrlEncode => {
                url_encode::run();
            }
        }
    }
}
