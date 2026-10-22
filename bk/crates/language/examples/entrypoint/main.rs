// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod async_main;
mod main_fn;
mod main_fn_with_result;

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
    /// Runs the `async_main` example.
    #[command(name = "async_main")]
    AsyncMain,
    /// Runs the `main_fn` example.
    #[command(name = "main_fn")]
    MainFn,
    /// Runs the `main_fn_with_result` example.
    #[command(name = "main_fn_with_result")]
    MainFnWithResult,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncMain => {
                async_main::run()?;
            }
            Commands::MainFn => {
                main_fn::run();
            }
            Commands::MainFnWithResult => {
                main_fn_with_result::run()?;
            }
        }
    }
    Ok(())
}
