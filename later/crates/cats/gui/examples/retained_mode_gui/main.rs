use clap::{Parser, Subcommand};

#[cfg(feature = "floem")]
mod floem;
mod iced;
mod slint;
#[cfg(feature = "vizia")]
mod vizia;
#[cfg(feature = "xilem")]
mod xilem;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "floem")]
    #[command(name = "floem")]
    Floem,
    #[command(name = "iced")]
    Iced,
    #[command(name = "slint")]
    Slint,
    #[cfg(feature = "vizia")]
    #[command(name = "vizia")]
    Vizia,
    #[cfg(feature = "xilem")]
    #[command(name = "xilem")]
    Xilem,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "floem")]
            Commands::Floem => {
                floem::run();
            }
            Commands::Iced => {
                iced::run();
            }
            Commands::Slint => {
                slint::run();
            }
            #[cfg(feature = "vizia")]
            Commands::Vizia => {
                vizia::run();
            }
            #[cfg(feature = "xilem")]
            Commands::Xilem => {
                xilem::run();
            }
        }
    }
    Ok(())
}

// [finish fix](https://github.com/john-cd/rust_howto/issues/1051)
