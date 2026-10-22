// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod js_sys;
mod web_sys;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `js_sys` example.
    #[command(name = "js_sys")]
    JsSys,
    /// Runs the `web_sys` example.
    #[command(name = "web_sys")]
    WebSys,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::JsSys => {
                js_sys::run();
            }
            Commands::WebSys => {
                web_sys::run();
            }
        }
    }
}
