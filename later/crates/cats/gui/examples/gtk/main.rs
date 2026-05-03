#![allow(unexpected_cfgs)]

#[cfg(feature = "gtk")]
use clap::Parser;
#[cfg(feature = "gtk")]
use clap::Subcommand;

#[cfg(feature = "gtk")]
mod gtk4;
#[cfg(feature = "gtk")]
mod relm4;

#[cfg(feature = "gtk")]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(feature = "gtk")]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "gtk")]
    #[command(name = "gtk4")]
    Gtk4,
    #[cfg(feature = "gtk")]
    #[command(name = "relm4")]
    Relm4,
}

fn main() {
    #[cfg(feature = "gtk")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "gtk")]
                Commands::Gtk4 => {
                    gtk4::run();
                }
                #[cfg(feature = "gtk")]
                Commands::Relm4 => {
                    relm4::run();
                }
            }
        }
    }
}
