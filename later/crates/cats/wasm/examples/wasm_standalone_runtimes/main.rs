use clap::{Parser, Subcommand};

mod wasmer;
#[cfg(feature = "wasmtime")]
mod wasmtime;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "wasmer")]
    Wasmer,
    #[cfg(feature = "wasmtime")]
    #[command(name = "wasmtime")]
    Wasmtime,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Wasmer => {
                let _ = wasmer::run();
            }
            #[cfg(feature = "wasmtime")]
            Commands::Wasmtime => {
                let _ = wasmtime::run();
            }
        }
    }
}
