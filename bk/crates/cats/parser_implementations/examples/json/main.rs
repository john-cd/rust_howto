use clap::Parser;
use clap::Subcommand;

mod json5;
mod serde_json;
mod simd_json;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "json5")]
    Json5,
    #[command(name = "serde_json")]
    SerdeJson,
    #[command(name = "simd_json")]
    SimdJson,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Json5 => {
                json5::run();
            }
            Commands::SerdeJson => {
                serde_json::run();
            }
            Commands::SimdJson => {
                simd_json::run();
            }
        }
    }
}
