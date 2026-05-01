use clap::Parser;
use clap::Subcommand;

mod bytes;
mod string_format;
mod strings;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bytes")]
    Bytes,
    #[command(name = "string_format")]
    StringFormat,
    #[command(name = "strings")]
    Strings,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bytes => {
                bytes::run();
            }
            Commands::StringFormat => {
                string_format::run();
            }
            Commands::Strings => {
                strings::run();
            }
        }
    }
}
