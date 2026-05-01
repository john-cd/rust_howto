use clap::Parser;
use clap::Subcommand;

mod async_main;
mod main_fn;
mod main_fn_with_result;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "async_main")]
    AsyncMain,
    #[command(name = "main_fn")]
    MainFn,
    #[command(name = "main_fn_with_result")]
    MainFnWithResult,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncMain => {
                let _ = async_main::run();
            }
            Commands::MainFn => {
                let _ = main_fn::run();
            }
            Commands::MainFnWithResult => {
                let _ = main_fn_with_result::run();
            }
        }
    }
}
