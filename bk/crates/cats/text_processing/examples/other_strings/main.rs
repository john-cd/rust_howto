use clap::Parser;
use clap::Subcommand;

mod bstr;
mod cstring;
mod osstring;
mod ustr;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bstr")]
    Bstr,
    #[command(name = "cstring")]
    Cstring,
    #[command(name = "osstring")]
    Osstring,
    #[command(name = "ustr")]
    Ustr,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bstr => {
                bstr::run();
            }
            Commands::Cstring => {
                cstring::run();
            }
            Commands::Osstring => {
                osstring::run();
            }
            Commands::Ustr => {
                ustr::run();
            }
        }
    }
}
