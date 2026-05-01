use clap::{Parser, Subcommand};

mod js_sys;
mod web_sys;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "js_sys")]
    JsSys,
    #[command(name = "web_sys")]
    WebSys,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::JsSys => {
                let _ = js_sys::run();
            }
            Commands::WebSys => {
                let _ = web_sys::run();
            }
        }
    }
}
