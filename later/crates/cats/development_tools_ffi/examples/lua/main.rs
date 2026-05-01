#[cfg(feature = "lua")]
use clap::{Parser, Subcommand};

#[cfg(feature = "lua")]
mod mlua;
#[cfg(feature = "lua")]
mod mlua2;

#[cfg(feature = "lua")]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(feature = "lua")]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "lua")]
    #[command(name = "mlua")]
    Mlua,
    #[cfg(feature = "lua")]
    #[command(name = "mlua2")]
    Mlua2,
}

fn main() {
    #[cfg(feature = "lua")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "lua")]
                Commands::Mlua => {
                    mlua::run();
                }
                #[cfg(feature = "lua")]
                Commands::Mlua2 => {
                    mlua2::run();
                }
            }
        }
    }
}
