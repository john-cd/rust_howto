// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod approx;
mod tests_custom_message;

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
    /// Runs the `approx` example.
    #[command(name = "approx")]
    Approx,
    /// Runs the `tests_custom_message` example.
    #[command(name = "tests_custom_message")]
    TestsCustomMessage,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Approx => {
                approx::run();
            }
            Commands::TestsCustomMessage => {
                tests_custom_message::run();
            }
        }
    }
}
