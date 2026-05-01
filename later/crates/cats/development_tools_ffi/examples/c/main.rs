use clap::{Parser, Subcommand};

mod bindgen;
mod cbindgen;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bindgen")]
    Bindgen,
    #[command(name = "cbindgen")]
    Cbindgen,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bindgen => {
                bindgen::run();
            }
            Commands::Cbindgen => {
                cbindgen::run();
            }
        }
    }
}
