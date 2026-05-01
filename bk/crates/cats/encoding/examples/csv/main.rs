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
                delimiter::run();
            }
            Commands::Filter => {
                filter::run();
            }
            Commands::Invalid => {
                invalid::run();
            }
            Commands::Read => {
                read::run();
            }
            Commands::Read1 => {
                read1::run();
            }
            Commands::SerdeSerialize => {
                serde_serialize::run();
            }
            Commands::Serialize => {
                serialize::run();
            }
            Commands::Transform => {
                transform::run();
            }
        }
    }
}
