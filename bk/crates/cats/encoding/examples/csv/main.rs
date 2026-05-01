use clap::Parser;
use clap::Subcommand;

mod delimiter;
mod filter;
mod invalid;
mod read;
mod read1;
mod serde_serialize;
mod serialize;
mod transform;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "delimiter")]
    Delimiter,
    #[command(name = "filter")]
    Filter,
    #[command(name = "invalid")]
    Invalid,
    #[command(name = "read")]
    Read,
    #[command(name = "read1")]
    Read1,
    #[command(name = "serde_serialize")]
    SerdeSerialize,
    #[command(name = "serialize")]
    Serialize,
    #[command(name = "transform")]
    Transform,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Delimiter => {
                let _ = delimiter::run();
            }
            Commands::Filter => {
                let _ = filter::run();
            }
            Commands::Invalid => {
                let _ = invalid::run();
            }
            Commands::Read => {
                let _ = read::run();
            }
            Commands::Read1 => {
                let _ = read1::run();
            }
            Commands::SerdeSerialize => {
                let _ = serde_serialize::run();
            }
            Commands::Serialize => {
                let _ = serialize::run();
            }
            Commands::Transform => {
                let _ = transform::run();
            }
        }
    }
}
