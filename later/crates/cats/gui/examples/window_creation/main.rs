use clap::{Parser, Subcommand};

mod baseview;
mod tao;
mod winit;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "baseview")]
    Baseview,
    #[command(name = "tao")]
    Tao,
    #[command(name = "winit")]
    Winit,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Baseview => {
                let _ = baseview::run();
            }
            Commands::Tao => {
                let _ = tao::run();
            }
            Commands::Winit => {
                let _ = winit::run();
            }
        }
    }
    Ok(())
}
