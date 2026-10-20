#![allow(unexpected_cfgs)]

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "floem")]
mod floem;
#[cfg(feature = "iced")]
mod iced;
#[cfg(feature = "slint")]
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
    #[cfg(feature = "iced")]
    #[command(name = "iced")]
    Iced,
    #[cfg(feature = "slint")]
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
            #[cfg(feature = "iced")]
            Commands::Iced => {
                iced::run();
            }
            #[cfg(feature = "slint")]
            Commands::Slint => {
                slint::run()?;
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
